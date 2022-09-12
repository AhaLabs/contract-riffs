use near_sdk::borsh::{BorshDeserialize, BorshSerialize};

pub mod lazy_option;
use crate::IntoKey;
pub use lazy_option::*;

/// Components meet the requirements to implement this trait.
/// Allows getting a lazy option of the riff state and setting it back to storage.
pub trait Lazy: Sized {
    fn get_lazy() -> Option<Self>;

    fn set_lazy(value: Self) -> Option<Self>;
}

/// Here we implement the trait for all riffs.
///
/// Adding this to the scope adds the methods to riff type
impl<Item> Lazy for Item
where
    Item: IntoKey + BorshDeserialize + BorshSerialize,
{
    fn get_lazy() -> Option<Self> {
        LazyOption::new(Self::into_storage_key(), None).get()
    }

    fn set_lazy(value: Self) -> Option<Self> {
        LazyOption::new(Self::into_storage_key(), None).replace(&value)
    }
}
