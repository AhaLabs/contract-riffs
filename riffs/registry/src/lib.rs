use near_riffs::prelude::*;
use near_riffs::{
    near_sdk::{
        self,
        borsh::{self, BorshDeserialize, BorshSerialize},
        collections::Vector,
        near_bindgen,
    },
    refund_storage_cost, reg,
    version::Version,
};

/// Bootloader riff
use near_riffs_core::Owner;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Registry {
    versions: Vector<Version>,
}

impl Default for Registry {
    fn default() -> Self {
        Self {
            versions: Vector::new(b"v"),
        }
    }
}

impl IntoKey for Registry {
    fn into_storage_key() -> Vec<u8> {
        "REGISTRY".as_bytes().to_vec()
    }
}

#[near_bindgen(riff)]
impl Registry {
    /// Non-breaking fix
    #[payable]
    pub fn patch(&mut self) {
        Owner::assert_owner();
        self.input_to_storage(self.current().publish_patch())
    }

    /// Non-breaking feature
    #[payable]
    pub fn minor(&mut self) {
        Owner::assert_owner();
        self.input_to_storage(self.current().publish_minor())
    }

    /// Breaking change
    #[payable]
    pub fn major(&mut self) {
        Owner::assert_owner();
        self.input_to_storage(self.current().publish_major())
    }

    /// Fetch a version of the contract
    /// If no argument provided use current version
    pub fn fetch(&self) {
        let value_reg = if reg::input_is_empty() {
            let key: Vec<u8> = (&self.current()).into();
            reg::storage_read(&key)
        } else {
            reg::storage_read_from_input()
        }
        .expect("MISSING BINARY");

        reg::value_return(value_reg);
    }

    /// Current version of the contract
    fn current(&self) -> Version {
        let len = self.versions.len();
        if len == 0 {
            Version::default()
        } else {
            self.versions
                .get(len - 1)
                .expect("failed to get current version")
        }
    }

    fn input_to_storage(&mut self, new_version: Version) {
        refund_storage_cost(|| {
            new_version.input_to_storage();
            self.versions.push(&new_version);
        })
    }

    /// Current version of the contract
    pub fn current_version(&self) -> String {
        self.current().to_string()
    }

    pub fn versions(&self) -> Vec<String> {
        self.versions
            .to_vec()
            .iter()
            .map(ToString::to_string)
            .collect()
    }
}

impl Registry {
    pub fn fetch_to_reg(&self) -> u64 {
        reg::storage_read(&self.current().to_key()).expect("Currently no version available")
    }
}
