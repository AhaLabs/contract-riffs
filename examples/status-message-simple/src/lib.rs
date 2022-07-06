//! # Status Message Contract
//!
//! This is an example contract using owneable and deployable components
//!

use near_components::{
    account_id_from_input,
    near_sdk::{
        self,
        borsh::{self, BorshDeserialize, BorshSerialize},
        env,
        serde::{Deserialize, Serialize},
        AccountId,
    },
};

/// Uses ownable to check owner before deploying contract
pub use near_components::prelude::*;
use near_components_core::AssertOwnable;
pub use near_components_core::{Deployable, Ownable};

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

// #[near_bindgen(riff)]
impl Ownable for Message {}

#[no_mangle]
fn set_owner() {
    let account_id = account_id_from_input();
    Message::set_owner(account_id)
}

#[no_mangle]
fn get_owner() {
    let result = &format!("\"{}\"", Message::get_owner());
    env::value_return(result.as_bytes())
}

#[no_mangle]
fn is_owner() {
    let res = Message::is_owner(account_id_from_input());
    env::value_return((if res { "true" } else { "false" }).as_bytes());
}

// #[near_bindgen(riff)]
impl Deployable for Message {}

#[no_mangle]
pub fn deploy() {
  Message::deploy();
}

#[no_mangle]
pub fn _deploy() {
  Message::_deploy();
}

#[no_mangle]
pub fn update_message() {
    Message::assert_owner();

    // Deserialize input into Message
    let msg: Message = near_sdk::serde_json::from_slice(
        &near_sdk::env::input().expect("Expected input since method has arguments."),
    )
    .expect("Failed to deserialize input from JSON.");

    // set new message and get old message
    let old_message = Message::set_lazy(msg);

    // Serialize old message
    let result = near_sdk::serde_json::to_vec(&old_message)
        .expect("Failed to serialize the return value using JSON.");

    // Return serailazed result
    near_sdk::env::value_return(&result)
}

#[no_mangle]
pub fn get_message() {
    // Get message instance from storage and fail if doesn't exist
    let message = Message::get_lazy().unwrap();

    // Serialize Message
    let result = near_sdk::serde_json::to_vec(&message)
        .expect("Failed to serialize the return value using JSON.");

    // Return serelaized result
    near_sdk::env::value_return(&result)
}
