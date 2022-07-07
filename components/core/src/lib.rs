use near_components::{
    near_sdk::{
        self,
        borsh::{self, BorshDeserialize, BorshSerialize},
        env, near_bindgen, require, AccountId,
    },
};

pub mod deploy;
pub use deploy::Deployable;

/// Uses ownable to check owner before deploying contract
pub use near_components::lazy::Lazy;
pub use near_components::IntoKey;

pub const OWNER_KEY: &str = "OWNER";

#[derive(BorshSerialize, BorshDeserialize)]
#[near_bindgen(component)]
pub struct Owner(AccountId);

// impl Lazy for Owner {
//     fn get_lazy() -> Option<Self> {
        
//     }

//     fn set_lazy(value: Self) -> Option<Self> {
//         todo!()
//     }
// }

impl IntoKey for Owner {
  fn into_storage_key() -> Vec<u8> {
      OWNER_KEY.as_bytes().to_vec()
  }
}

pub trait AssertOwnable {
    fn assert_owner();
    fn predecessor_is_owner() -> bool;
}

impl<Item: Ownable> AssertOwnable for Item {
    fn assert_owner() {
        require!(Self::predecessor_is_owner(), "Predecessor is not owner")
    }

    fn predecessor_is_owner() -> bool {
        Self::is_owner(env::predecessor_account_id())
    }
}

// #[witgen]
pub trait Ownable {
    /// @change
    fn set_owner(account_id: AccountId) {
        Owner::set_lazy(Owner(account_id));
    }

    fn get_owner() -> AccountId {
        Owner::get_lazy().unwrap().0
    }

    fn is_owner(account_id: AccountId) -> bool {
        Self::get_owner() == account_id
    }
}

impl Ownable for Owner {}
