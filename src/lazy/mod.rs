use near_sdk::{
    borsh::{BorshDeserialize, BorshSerialize},
    env,
};

use crate::IntoKey;

const ERR_VALUE_SERIALIZATION: &str = "Cannot serialize value with Borsh";
const ERR_VALUE_DESERIALIZATION: &str = "Cannot deserialize value with Borsh";

/// Components meet the requirements to implement this trait.
/// Allows getting a lazy option of the riff state and setting it back to storage.
pub trait Lazy: Sized {
    /// Get the singleton of the type
    fn get_lazy() -> Option<Self>;

    /// Write the singleton
    fn set_lazy(value: Self) -> Option<Self>;
}

/// Here we implement the trait for all riffs.
///
/// Adding this to the scope adds the methods to riff type
impl<Item> Lazy for Item
where
    Item: IntoKey + BorshDeserialize + BorshSerialize + Default,
{
    fn get_lazy() -> Option<Self> {
        storage_read::<Item>().as_deref().map(deserialize)
    }

    fn set_lazy(value: Self) -> Option<Self> {
        storage_write(value).as_deref().map(deserialize)
    }
}

fn serialize<T: BorshSerialize>(t: T) -> Vec<u8> {
    t.try_to_vec()
        .unwrap_or_else(|_| env::panic_str(ERR_VALUE_SERIALIZATION))
}

fn deserialize<T: BorshDeserialize>(t: &[u8]) -> T {
    T::try_from_slice(t).unwrap_or_else(|_| env::panic_str(ERR_VALUE_DESERIALIZATION))
}

fn storage_read<T>() -> Option<Vec<u8>>
where
    T: IntoKey,
{
    env::storage_read(&T::into_storage_key())
}

fn storage_write<T>(t: T) -> Option<Vec<u8>>
where
    T: BorshSerialize + IntoKey,
{
    env::storage_write(&T::into_storage_key(), &serialize(t))
        .then(env::storage_get_evicted)
        .flatten()
}
