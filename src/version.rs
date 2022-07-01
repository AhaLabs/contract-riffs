use std::fmt::Display;

use crate::{
    near_sdk::{
        borsh::{self, BorshDeserialize, BorshSerialize},
        serde::Serialize,
    },
    reg,
};

/// Represents the version of the contract
#[derive(BorshSerialize, BorshDeserialize, Default)]
pub struct Version {
    patch: u16,
    minor: u16,
    major: u16,
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: near_sdk::serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v{}_{}_{}", self.major, self.minor, self.patch)
    }
}

impl Version {
    pub fn publish_patch(mut self) -> Self {
        self.patch += 1;
        self
    }

    pub fn publish_minor(mut self) -> Self {
        self.minor += 1;
        self.patch = 0;
        self
    }
    pub fn publish_major(mut self) -> Self {
        self.major += 1;
        self.minor = 0;
        self.patch = 0;
        self
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
