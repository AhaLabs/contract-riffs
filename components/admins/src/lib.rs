use near_components::{
    lazy::Lazy,
    near_sdk::{
        self,
        borsh::{self, BorshDeserialize, BorshSerialize},
        env, near_bindgen, require, AccountId,
    },
    IntoKey,
};

pub use near_components_core::{Ownable, AssertOwnable};

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
    fn assert_admin();
    fn predecessor_is_admin() -> bool;
}

impl<Item: Administratable> AdminAssertable for Item {
    fn assert_admin() {
        require!(Self::predecessor_is_admin(), "Not allowed: must be admin");
    }

    fn predecessor_is_admin() -> bool {
        Self::is_admin(env::predecessor_account_id())
    }
}

pub trait AdminAndOwnerAssertable {
    fn assert_owner_or_admin();
}

impl<Item: AdminAssertable + AssertOwnable> AdminAndOwnerAssertable for Item {
    fn assert_owner_or_admin() {
        require!(
            Self::predecessor_is_admin() || Self::predecessor_is_owner(),
            "Not allowed: must be owner or admin"
        );
    }
}

// #[default(Admins)]
pub trait Administratable {
    fn add_admin(account_id: AccountId) {
        let mut admins = Admins::get_lazy().unwrap_or_default();
        admins.add_admin(account_id);
        Admins::set_lazy(admins);
    }

    fn get_admins() -> Vec<AccountId> {
        Admins::get_lazy().unwrap_or_default().get_admins()
    }

    fn is_admin(account_id: AccountId) -> bool {
        Admins::get_lazy()
            .unwrap_or_default()
            .is_admin(account_id)
    }
}

pub trait OwnableAdminster: Administratable + Ownable {}

impl Administratable for Admins {}
impl Ownable for Admins {}

// #[generate_trait]
impl Admins {
    fn add_admin(&mut self, account_id: AccountId) {
        Self::assert_owner_or_admin();
        self.admins.push(account_id);
    }

    fn get_admins(&self) -> Vec<AccountId> {
        self.admins.clone()
    }

    fn is_admin(&self, account_id: AccountId) -> bool {
        self.admins.contains(&account_id)
    }
}
