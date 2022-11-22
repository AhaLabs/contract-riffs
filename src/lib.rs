//! # Near Components
//!
//! Composible riffs for NEAR smart contracts

pub use near_sdk;
pub use near_units;
pub use witgen::witgen;

pub mod account;
pub mod lazy;
pub mod promise;
pub mod reg;
pub mod version;
pub mod input;

use near_sdk::{env, require, AccountId};

pub mod prelude {
    pub use super::lazy::Lazy;
    pub use super::IntoKey;
}

/// Mesaure cost of storage for a function
pub fn measure_storage_cost<O, F: FnOnce() -> O>(f: F) -> (O, u128) {
    let bytes_used_before = env::storage_usage();
    let res = f();
    let bytes_used = env::storage_usage() - bytes_used_before;
    (res, env::storage_byte_cost() * bytes_used as u128)
}

pub fn left_over_balance<O, F: FnOnce() -> O>(f: F) -> (O, u128) {
    let (res, cost) = measure_storage_cost(f);
    let attached_deposit = near_sdk::env::attached_deposit();
    require!(
        attached_deposit >= cost,
        "Not enough attached deposit to cover storage"
    );
    (res, attached_deposit - cost)
}

/// Excute function f, then transfer funds left over after charging for storage stake.
/// ```ignore
/// pub fn contract_method(&mut self, item: Item) {
/// refund_storage_cost(|| {
///     self.push(&item)
/// })
/// }
/// ```
pub fn refund_storage_cost<O, F: FnOnce() -> O>(f: F) -> O {
    let (res, amount_to_refund) = left_over_balance(f);
    if 0 < amount_to_refund {
        let promise_index = reg::promise_batch_create_for_predecessor();
        env::promise_batch_action_transfer(promise_index, amount_to_refund);
    }
    res
}

pub trait IntoKey {
    fn into_storage_key() -> Vec<u8>;
}

pub fn input_as_str() -> String {
    unsafe { String::from_utf8_unchecked(env::input().unwrap()) }
}

/// Can decode `{"account_id": account_id}`, `"account_id"`, or `account_id`
pub fn account_id_from_input() -> AccountId {
    let input = input_as_str();
    input.parse().unwrap_or_else(|_| {
        parse_json_or_string(input.as_str(), "account_id")
            .unwrap()
            .parse()
            .unwrap()
    })
}

pub fn parse_json_or_string(input: &str, key: &str) -> Result<String, microjson::JSONParsingError> {
    use microjson::JSONValue;
    let object = JSONValue::parse(input)?;
    use microjson::JSONValueType;
    match object.value_type {
        JSONValueType::String => object.read_string().map(Into::into),
        JSONValueType::Object => object
            .get_key_value(key)
            .and_then(|val| val.read_string().map(ToString::to_string)),
        _ => env::panic_str("cannot parse account_id"),
    }
}
