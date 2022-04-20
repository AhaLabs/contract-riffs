
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};


use crate::env;
use crate::near_sdk::IntoStorageKey;


const ERR_VALUE_SERIALIZATION: &str = "Cannot serialize value with Borsh";
const ERR_VALUE_DESERIALIZATION: &str = "Cannot deserialize value with Borsh";
const ERR_NOT_FOUND: &str = "No value found for the given key";
const ERR_DELETED: &str = "The Lazy cell's value has been deleted. Verify the key has not been\
                            deleted manually.";

fn expect_key_exists<T>(val: Option<T>) -> T {
    val.unwrap_or_else(|| env::panic_str(ERR_NOT_FOUND))
}

fn expect_consistent_state<T>(val: Option<T>) -> T {
    val.unwrap_or_else(|| env::panic_str(ERR_DELETED))
}

pub(crate) fn load_and_deserialize<T>(key: &[u8]) -> Option<T>
where
    T: BorshDeserialize,
{
    let bytes = expect_key_exists(env::storage_read(key));
    let val =
        T::try_from_slice(&bytes).unwrap_or_else(|_| env::panic_str(ERR_VALUE_DESERIALIZATION));
    Some(val)
}

pub(crate) fn serialize_and_store<T>(key: &[u8], value: &T)
where
    T: BorshSerialize,
{
    let serialized = value.try_to_vec().unwrap_or_else(|_| env::panic_str(ERR_VALUE_SERIALIZATION));
    env::storage_write(key, &serialized);
}

/// An persistent lazily loaded option, that stores a `value` in the storage when `Some(value)`
/// is set, and not when `None` is set. `LazyOption` also [`Deref`]s into [`Option`] so we get
/// all its APIs for free.
///
/// This will only write to the underlying store if the value has changed, and will only read the
/// existing value from storage once.
///
/// # Examples
/// ```
/// use near_sdk::store::LazyOption;
///
/// let mut a = LazyOption::new(b"a", None);
/// assert!(a.is_none());
///
/// *a = Some("new value".to_owned());
/// assert_eq!(a.get(), &Some("new value".to_owned()));
///
/// // Using Option::replace:
/// let old_str = a.replace("new new value".to_owned());
/// assert_eq!(old_str, Some("new value".to_owned()));
/// assert_eq!(a.get(), &Some("new new value".to_owned()));
/// ```
/// [`Deref`]: std::ops::Deref
#[derive(BorshSerialize, BorshDeserialize)]
pub struct LazyOption<T>
where
    T: BorshSerialize,
{
    /// Key bytes to index the contract's storage.
    storage_key: Box<[u8]>,

}

trait HasKey {
  fn get() -> Box<[u8]>;
}

impl<T> LazyOption<T>
where
    T: BorshSerialize,
{
    

    /// Updates the value with a new value. This does not load the current value from storage.
    pub fn set<K: HashKey>(key: K, value: T) {

    }

    /// Writes any changes to the value to storage. This will automatically be done when the
    /// value is dropped through [`Drop`] so this should only be used when the changes need to be
    /// reflected in the underlying storage before then.
    pub fn flush(&mut self) {
        if let Some(v) = self.cache.get_mut() {
            if !v.is_modified() {
                return;
            }

            match v.value().as_ref() {
                Some(value) => serialize_and_store(&self.storage_key, value),
                None => {
                    env::storage_remove(&self.storage_key);
                }
            }

            // Replaces cache entry state to cached because the value in memory matches the
            // stored value. This avoids writing the same value twice.
            v.replace_state(EntryState::Cached);
        }
    }
}

impl<T> LazyOption<T>
where
    T: BorshSerialize + BorshDeserialize,
{
    /// Returns a reference to the lazily loaded optional.
    /// The load from storage only happens once, and if the value is already cached, it will not
    /// be reloaded.
    pub fn get(&self) -> &Option<T> {
        let entry = self.cache.get_or_init(|| load_and_deserialize(&self.storage_key));
        entry.value()
    }

    /// Returns a reference to the lazily loaded optional.
    /// The load from storage only happens once, and if the value is already cached, it will not
    /// be reloaded.
    pub fn get_mut(&mut self) -> &mut Option<T> {
        self.cache.get_or_init(|| load_and_deserialize(&self.storage_key));
        let entry = self.cache.get_mut().unwrap_or_else(|| env::abort());
        entry.value_mut()
    }
}
