//! # Status Message Contract
//!
//! This is an example contract using owneable and deployable riffs
//!

use near_riffs::{
    near_sdk::{
        self,
        borsh::{self, BorshDeserialize, BorshSerialize},
        near_bindgen,
        serde::{Deserialize, Serialize},
    },
    witgen,
};

/// Uses ownable to check owner before deploying contract
pub use near_riffs::prelude::*;
pub use near_riffs_admins::Owner;
// pub use near_riffs::{Administratable, Ownable};

const MESSAGE_KEY: &str = "MESSAGE";

#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Default)]
#[serde(crate = "near_sdk::serde")]
#[witgen]
#[near_bindgen(riff)]
pub struct Message {
    text: String,
}

// TODO: Make derivable
// impl Administratable for Message {
//     fn predecessor_is_admin(&self) -> bool {
//         true
//     }
//
//     pub fn is_admin(&self, account_id: AccountId) -> bool {
//         true
//     }
// }

// impl Administratable for Message {}

impl IntoKey for Message {
    fn into_storage_key() -> Vec<u8> {
        MESSAGE_KEY.as_bytes().to_vec()
    }
}

#[near_bindgen(riff)]
impl Message {
    pub fn update_message(&mut self, message: Message) -> Message {
        Owner::assert_owner();
        // set new message and get old message
        let mut message = message;
        std::mem::swap(self, &mut message);
        message
    }

    pub fn get_message(self) -> Message {
        self
    }
}
