//! # Status Message Contract
//!
//! This is an example contract using owneable and deployable components
//!

use contract_utils::{
    near_sdk::{
        self,
        borsh::{self, BorshDeserialize, BorshSerialize},
        serde::{Deserialize, Serialize},
    },
    IntoKey,
};

/// Is ownable, e.i. stores owner in storage at "OWNER"
pub use contract_utils::owner::*;
/// Uses ownable to check owner before deploying contract
pub use contract_utils::upgrade::*;

const MESSAGE_KEY: &str = "MESSAGE";

#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Default)]
#[serde(crate = "near_sdk::serde")]
pub struct Message {
    text: String,
}

impl IntoKey for Message {
    fn into_storage_key(&self) -> Vec<u8> {
        MESSAGE_KEY.as_bytes().to_vec()
    }
}

impl Message {
    fn update(&mut self, msg: Message) -> Option<String> {
        self.set_lazy(msg).as_ref().map(|m| m.text.clone())
    }
}

#[no_mangle]
pub fn update_message() {
    let msg: Message = near_sdk::serde_json::from_slice(
        &near_sdk::env::input().expect("Expected input since method has arguments."),
    )
    .expect("Failed to deserialize input from JSON.");
    let result = Message::default().update(msg);
    let result = near_sdk::serde_json::to_vec(&result)
        .expect("Failed to serialize the return value using JSON.");
    near_sdk::env::value_return(&result)
}

#[no_mangle]
pub fn get_message() {
    let result = Message::default().get_lazy().get();
    let result = near_sdk::serde_json::to_vec(&result)
        .expect("Failed to serialize the return value using JSON.");
    near_sdk::env::value_return(&result)
}
