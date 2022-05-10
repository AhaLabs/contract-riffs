//! # Status Message Contract
//!
//! This is an example contract using owneable and deployable components
//!

use contract_utils::{
    lazy::*,
    near_sdk::{
        self,
        borsh::{self, BorshDeserialize, BorshSerialize},
        serde::{Deserialize, Serialize},
    },
    IntoKey,
};

/// Uses ownable to check owner before deploying contract
pub use contract_utils::deploy::*;
/// Is ownable, e.i. stores owner in storage at "OWNER"
pub use contract_utils::owner::*;

const MESSAGE_KEY: &str = "MESSAGE";

#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Default)]
#[serde(crate = "near_sdk::serde")]
pub struct Message {
    text: String,
}

impl IntoKey for Message {
    fn into_storage_key() -> Vec<u8> {
        MESSAGE_KEY.as_bytes().to_vec()
    }
}

#[no_mangle]
pub fn update_message() {
    let msg: Message = near_sdk::serde_json::from_slice(
        &near_sdk::env::input().expect("Expected input since method has arguments."),
    )
    .expect("Failed to deserialize input from JSON.");
    let result = Message::set_lazy(msg);
    let result = near_sdk::serde_json::to_vec(&result)
        .expect("Failed to serialize the return value using JSON.");
    near_sdk::env::value_return(&result)
}

#[no_mangle]
pub fn get_message() {
    let result = Message::get_lazy().get();
    let result = near_sdk::serde_json::to_vec(&result)
        .expect("Failed to serialize the return value using JSON.");
    near_sdk::env::value_return(&result)
}
