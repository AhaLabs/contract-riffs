use near_components::{
    lazy::Lazy,
    near_sdk::{
        self,
        borsh::{self, BorshDeserialize, BorshSerialize},
        env, near_bindgen, require, AccountId,
    },
    IntoKey,
};

pub use near_components_core::*;

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

pub trait AdminAssertable {
    fn assert_admin(&self) {
        require!(self.predecessor_is_admin(), "Not allowed: must be admin");
    }

    fn predecessor_is_admin(&self) -> bool {
        Admins::get_lazy()
            .get()
            .unwrap_or_default()
            .is_admin(env::predecessor_account_id())
    }
}

pub trait AdminAndOwnerAssertable {
    fn assert_owner_or_admin(&self) {
        require!(
            self.predecessor_is_admin() || self.predecessor_is_owner(),
            "Not allowed: must be owner or admin"
        );
    }
}

pub trait Administratable {
    pub fn is_admin(&self, account_id: AccountId) -> bool {
        self.admins.contains(&account_id)
    }
}

impl Administratable for Admins {}

impl Admins {
    pub fn add_admin(&mut self, account_id: AccountId) {
        self.assert_owner_or_admin();
        self.admins.push(account_id);
    }

    pub fn get_admins(&self) -> Vec<AccountId> {
        self.admins.clone()
    }
}
