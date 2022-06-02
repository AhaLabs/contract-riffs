use contract_utils::{
    near_sdk::{
        self,
        borsh::{self, BorshDeserialize, BorshSerialize},
        collections::Vector,
        near_bindgen,
    },
    owner::*,
    publish::Version,
    refund_storage_cost, reg,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    current_version: Version,
    versions: Vector<Version>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            current_version: Default::default(),
            versions: Vector::new(b"v"),
        }
    }
}

#[near_bindgen]
impl Contract {
    /// Non-breaking fix
    #[payable]
    pub fn patch(&mut self) {
        self.assert_owner();
        self.current_version.publish_patch();
        refund_storage_cost(
            || {
                self.current_version.input_to_storage();
            },
            10,
        );
    }

    /// Non-breaking feature
    #[payable]
    pub fn minor(&mut self) {
        self.assert_owner();
        self.current_version.publish_minor();
        refund_storage_cost(
            || {
                self.current_version.input_to_storage();
            },
            10,
        );
    }

    /// Breaking change
    #[payable]
    pub fn major(&mut self) {
        self.assert_owner();
        self.current_version.publish_major();
        refund_storage_cost(
            || {
                self.current_version.input_to_storage();
            },
            10,
        );
    }

    pub fn fetch(&self) {
        let input_reg = 0;
        reg::input(input_reg);

        let value_reg = if reg::length(input_reg) == 0 {
            let key = self.current_version.to_key();
            reg::storage_read(&key, 1).expect("MISSING BINARY")
        } else {
            reg::storage_read_from_reg(input_reg, 1).expect("MISSING BINARY")
        };

        reg::value_return(value_reg);
    }

    pub fn current_version(&self) -> String {
        self.current_version.to_string()
    }
}
