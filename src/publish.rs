//! # Status Message Contract
//!
//! This is an example contract using owneable and deployable components
//!
use std::fmt::Display;

use crate::{
    deploy::promise::promise_create_proxy,
    near_sdk::{
        self,
        borsh::{self, BorshDeserialize, BorshSerialize},
        env,
        serde::{Deserialize, Serialize},
    },
    owner::Owner,
    reg,
};

use near_sdk::Gas;
use near_units::parse_gas;

const ONE_TGAS: Gas = Gas(parse_gas!("1 Tgas") as u64);

/// Represents the version of the contract
#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Default)]
#[serde(crate = "near_sdk::serde")]
pub struct Version {
    patch: u16,
    minor: u16,
    major: u16,
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v{}_{}_{}", self.major, self.minor, self.patch)
    }
}

impl Version {
    pub fn publish_patch(&mut self) {
        self.patch += 1
    }

    pub fn publish_minor(&mut self) {
        self.minor += 1;
        self.patch = 0
    }
    pub fn publish_major(&mut self) {
        self.major += 1;
        self.minor = 0;
        self.patch = 0
    }

    pub fn input_to_storage(&self) {
        let key = self.to_key();
        reg::storage_write_from_input(&key);
    }

    pub fn to_key(&self) -> Vec<u8> {
        format!("{}_{}_{}", self.major, self.minor, self.patch)
            .as_bytes()
            .to_vec()
    }
}

#[no_mangle]
pub fn publish_patch() {
    Owner::assert();
    promise_create_proxy(
        &format!("registry.{}", env::current_account_id()),
        "patch",
        env::attached_deposit(),
        // Subtract 1 Tgas just as a buffer
        env::prepaid_gas() - (env::used_gas() + ONE_TGAS),
    );
}
