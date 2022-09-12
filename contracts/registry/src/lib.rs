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

pub use near_riffs_core::*;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    versions: Vector<Version>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            versions: Vector::new(b"v"),
        }
    }
}

#[near_bindgen]
impl Contract {
    /// Non-breaking fix
    #[payable]
    pub fn patch(&mut self) {
        // self.assert_owner();
        self.input_to_storage(self.current().publish_patch())
    }

    /// Non-breaking feature
    #[payable]
    pub fn minor(&mut self) {
        // self.assert_owner();
        self.input_to_storage(self.current().publish_minor())
    }

    /// Breaking change
    #[payable]
    pub fn major(&mut self) {
        // self.assert_owner();
        self.input_to_storage(self.current().publish_major())
    }

    /// Fetch a version of the contract
    /// If no argument provided use current version
    pub fn fetch(&self) {
        let value_reg = if reg::input_len() == 0 {
            let key = self.current().to_key();
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
}
