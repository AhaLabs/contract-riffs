//! # Status Message Contract
//!
//! This is an example contract using owneable and deployable components
//!

use near_components::{
    account::{self, FixedAccountId},
    near_sdk::{
        self,
        borsh::{self, BorshDeserialize, BorshSerialize},
        env, near_bindgen, require,
        serde::{Deserialize, Serialize},
        serde_json, AccountId,
    },
    reg, witgen, account_id_from_input,
};

pub mod deploy;

/// Uses ownable to check owner before deploying contract
pub use near_components::lazy::Lazy;
pub use near_components::IntoKey;

pub const OWNER_KEY: &str = "OWNER";

pub struct Owner();

pub trait Ownable {
    fn assert_owner(&self) {
        Owner::assert()
    }

    fn predecessor_is_owner(&self) -> bool {
        Owner::is_predecessor()
    }
}

impl<Item> Ownable for Item {}

impl Owner {
    pub fn is_set() -> bool {
        env::storage_has_key(OWNER_KEY.as_bytes())
    }

    pub fn assert() {
        require!(Owner::is_predecessor(), "Predecessor is not owner");
    }
    pub fn is_predecessor() -> bool {
        let predecessor_account_id = account::predecessor_account_id();
        Owner::get().unwrap() == predecessor_account_id
    }

    fn set(account_id: &AccountId) {
        if Owner::is_set() {
            Owner::assert()
        };
        env::storage_write(OWNER_KEY.as_bytes(), account_id.as_bytes());
    }

    pub fn get() -> Option<FixedAccountId> {
        Owner::is_set()
            .then(|| account::read_register(reg::storage_read(OWNER_KEY.as_bytes()).unwrap()))
    }

    pub fn get_str() -> String {
        Owner::is_set()
            .then(|| unsafe {
                String::from_utf8_unchecked(env::storage_read(OWNER_KEY.as_bytes()).unwrap())
            })
            .unwrap_or_else(|| env::panic_str("Owner not set"))
    }
}

#[no_mangle]
pub fn set_owner() {
    Owner::set(&account_id_from_input());
}

#[no_mangle]
pub fn get_owner() {
    let s = &format!("\"{}\"", Owner::get_str());
    env::value_return(s.as_bytes())
}
