use crate::{account, component::lazy::LazyOption, reg, IntoKey};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, require,
};

pub const OWNER_KEY: &str = "OWNER";
type FixedAccountId = [u8; 64];
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Owner(FixedAccountId);

impl IntoKey for Owner {
    fn into_storage_key(&self) -> Vec<u8> {
        OWNER_KEY.as_bytes().to_vec()
    }
}

pub trait Lazy: Sized + BorshDeserialize + BorshSerialize {
    fn get_lazy(&self) -> LazyOption<Self>;

    fn set_lazy(&mut self, t: Self) -> Option<Self>;

    fn map<F: FnOnce(Self) -> U, U>(self, f: F) -> Option<U> {
        self.get_lazy().get().map(f)
    }
}

impl<Item> Lazy for Item
where
    Item: IntoKey + BorshDeserialize + BorshSerialize,
{
    fn get_lazy(&self) -> LazyOption<Self> {
        LazyOption::new(self.into_storage_key(), None)
    }

    fn set_lazy(&mut self, t: Self) -> Option<Self> {
        self.get_lazy().replace(&t)
    }
}

pub trait Ownable {
    fn assert_predecessor(&self) {
        let owner = Owner::get();
        require!(owner.is_some(), "No owner set");
        require!(owner.get().unwrap().is_predecessor());
    }
}

impl<Item> Ownable for Item {}

impl Owner {
    pub fn assert() {
        require!(Owner::get().get().unwrap().is_predecessor())
    }
    pub fn is_predecessor(&self) -> bool {
        let predecessor_account_id = account::predecessor_account_id();
        self.0 == predecessor_account_id
    }

    fn set_from_input() -> bool {
        let mut container = Owner::get();
        if let Some(owner) = container.get() {
            require!(owner.is_predecessor(), "must be owner")
        };
        container.set_reg(reg::input(0))
    }

    pub fn get() -> LazyOption<Owner> {
        LazyOption::new(OWNER_KEY.as_bytes(), None as Option<&Owner>)
    }
}

// pub fn get_owner() -> LazyOption<Owner> {
//     LazyOption::new(OWNER_KEY.as_bytes(), None as Option<&Owner>)
// }

#[no_mangle]
pub fn set_owner() {
    Owner::set_from_input();
}

#[no_mangle]
pub fn get_owner() {
    env::value_return(&Owner::get().get().unwrap().0)
}
