use near_components::prelude::*;

use near_components::near_sdk::{
    self,
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, near_bindgen, require, AccountId,
};

pub use near_components_core::{Owner};

const ADMINS_KEY: &str = "ADMINS";

#[derive(BorshSerialize, BorshDeserialize, Default)]
#[near_bindgen(component)]
pub struct Admins {
    admins: Vec<AccountId>,
}

impl IntoKey for Admins {
    fn into_storage_key() -> Vec<u8> {
        ADMINS_KEY.as_bytes().to_vec()
    }
}




impl Admins {
    pub fn assert_owner_or_admin() {
      let this = Admins::get_lazy().unwrap();
      let pred = env::predecessor_account_id();
        require!(
            this.is_admin(&pred) || Owner::get_lazy().unwrap().is_owner(pred),
            "Not allowed: must be owner or admin"
        );
    }
}

#[near_bindgen(riff)]
impl Admins {
    pub fn add_admin(&mut self, account_id: AccountId) {
        Self::assert_owner_or_admin();
        self.admins.push(account_id);
    }

    pub fn get_admins(&self) -> Vec<AccountId> {
        self.admins.clone()
    }

    pub fn is_admin(&self, account_id: &AccountId) -> bool {
        self.admins.contains(account_id)
    }
}
