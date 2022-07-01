//! # Status Message Contract
//!
//! This is an example contract using owneable and deployable components
//!

use near_components::{
    near_sdk::{
        self,
        borsh::{self, BorshDeserialize, BorshSerialize},
        near_bindgen,
        serde::{Deserialize, Serialize},
    },
    witgen,
};

/// Uses ownable to check owner before deploying contract
pub use near_components::prelude::*;

const MESSAGE_KEY: &str = "MESSAGE";

#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Default)]
#[serde(crate = "near_sdk::serde")]
#[witgen]
#[near_bindgen(component)]
pub struct Message {
    text: String,
}

impl IntoKey for Message {
    fn into_storage_key() -> Vec<u8> {
        MESSAGE_KEY.as_bytes().to_vec()
    }
}

#[near_bindgen(component)]
impl Message {
    pub fn update_message(&mut self, message: Message) -> Message {
        // self.assert_owner();
        // set new message and get old message
        let mut message = message;
        std::mem::swap(self, &mut message);
        message
    }

    pub fn get_message(self) -> Message {
        self
    }
}
