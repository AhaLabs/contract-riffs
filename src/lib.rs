//! # Near Components
//! 
//! Composible components for NEAR smart contracts


pub use near_sdk;
pub use near_units;
pub use witgen::witgen;

pub mod account;
pub mod lazy;
pub mod reg;
pub mod promise;
pub mod version;


use near_sdk::{env, require, AccountId};

pub mod prelude {
    pub use super::lazy::Lazy;
    pub use super::IntoKey;
}

/// Mesaure cost
pub fn measure_storage_cost<F: FnOnce()>(f: F) -> u128 {
    let bytes_used_before = env::storage_usage();
    f();
    let bytes_used = env::storage_usage() - bytes_used_before;
    env::storage_byte_cost() * bytes_used as u128
}

pub fn left_over_balance<F: FnOnce()>(f: F) -> u128 {
    let cost = measure_storage_cost(f);
    let attached_deposit = near_sdk::env::attached_deposit();
    require!(
        attached_deposit >= cost,
        "Not enough attached deposit to cover storage"
    );
    attached_deposit - cost
}

pub fn refund_storage_cost<F: FnOnce()>(f: F) {
    let amount_to_refund = left_over_balance(f);
    if 0 < amount_to_refund {
        let promise_index = reg::promise_batch_create_for_predecessor();
        env::promise_batch_action_transfer(promise_index, amount_to_refund)
    }
}

pub trait IntoKey {
    fn into_storage_key() -> Vec<u8>;
}

/// Can decode `{"account_id": account_id}`, `"account_id"`, or `account_id`
pub fn account_id_from_input() -> AccountId {
  use microjson::JSONValue;
  let input: String = unsafe { String::from_utf8_unchecked(env::input().unwrap()) };
  input.parse().unwrap_or_else(|_| {
      let object = JSONValue::parse(&input).unwrap();
      use microjson::JSONValueType;
      let account_id = match object.value_type {
          JSONValueType::String => object.read_string().map(Into::into),
          JSONValueType::Object => object
              .get_key_value("account_id")
              .and_then(|val| val.read_string().map(|x| x.to_string())),
          _ => env::panic_str("cannot parse account_id"),
      };
      account_id.unwrap().parse().unwrap()
  })
}