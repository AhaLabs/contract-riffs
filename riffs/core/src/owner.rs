pub use near_riffs::lazy::Lazy;
pub use near_riffs::IntoKey;
use near_riffs::{
    account_id_from_input,
    near_sdk::{
        self, assert_one_yocto,
        borsh::{self, BorshDeserialize, BorshSerialize},
        env, near_bindgen, require, AccountId,
    },
};

pub const OWNER_KEY: &str = "OWNER";

#[derive(BorshSerialize, BorshDeserialize, Default)]
#[near_bindgen(riff)]
pub struct Owner(Option<AccountId>);

// impl Default for Owner {
//     fn default() -> Self {
//         Self(None)
//     }
// }

impl IntoKey for Owner {
    fn into_storage_key() -> Vec<u8> {
        OWNER_KEY.as_bytes().to_vec()
    }
}

impl Owner {
    pub fn assert_owner() {
        require!(Self::predecessor_is_owner(), "Predecessor is not owner")
    }

    pub fn assert_with_one_yocto() {
        Owner::assert_owner();
        assert_one_yocto()
    }

    pub fn predecessor_is_owner() -> bool {
        Owner::get_lazy()
            .unwrap()
            .is_owner(env::predecessor_account_id())
    }
}

#[near_bindgen(riff)]
impl Owner {
    /// Hello
    /// @change
    pub fn set_owner(&mut self) {
        if let Some(owner) = &self.0 {
            require!(
                *owner == env::predecessor_account_id(),
                "only owner can transfer ownership"
            )
        }
        let account_id = account_id_from_input();
        self.0 = Some(account_id);
    }

    pub fn get_owner(&self) {
        env::value_return(self.0.as_ref().unwrap().as_bytes())
    }

    pub fn get_owner_json(&self) {
        env::value_return(format!("\"{}\"", self.0.as_ref().unwrap()).as_bytes())
    }

    pub fn is_owner(self, account_id: AccountId) -> bool {
        self.0.unwrap() == account_id
    }
}

// #[no_mangle]
// pub fn set_owner() {
//     let mut owner = Owner::get_lazy().unwrap_or_default();
//     owner.set_owner();
//     Owner::set_lazy(owner);
// }

// #[no_mangle]
// pub fn get_owner() {
//   Owner::get_lazy().unwrap().get_owner()
// }

// #[no_mangle]
// pub fn get_owner_json() {
//   Owner::get_lazy().unwrap().get_owner_json()
// }
