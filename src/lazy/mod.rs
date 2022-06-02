use near_sdk::borsh::{BorshDeserialize, BorshSerialize};

pub mod lazy_option;
use crate::IntoKey;
pub use lazy_option::*;

pub trait Lazy: Sized + BorshDeserialize + BorshSerialize + IntoKey {
    fn get_lazy() -> LazyOption<Self>;

    fn set_lazy(value: Self) -> Option<Self>;

    fn map<F: FnOnce(Self) -> U, U>(f: F) -> Option<U> {
        Self::get_lazy().get().map(f)
    }

    fn mut_map<F: FnOnce(Self) -> Self>(f: F) -> Option<Self> {
        Self::get_lazy().mut_map(f)
    }

    fn mut_map_or_else<D, F>(default: D, f: F) -> Option<Self>
    where
        D: FnOnce() -> Self,
        F: FnOnce(Self) -> Self,
    {
        Self::get_lazy().mut_map_or_else(default, f)
    }
}

impl<Item> Lazy for Item
where
    Item: IntoKey + BorshDeserialize + BorshSerialize,
{
    fn get_lazy() -> LazyOption<Self> {
        LazyOption::new(Self::into_storage_key(), None)
    }

    fn set_lazy(value: Self) -> Option<Self> {
        Self::get_lazy().replace(&value)
    }
}
