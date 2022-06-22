pub use near_sdk;
pub use near_units;
pub mod account;
pub mod deploy;
pub mod lazy;
pub mod owner;
pub mod publish;
pub mod reg;

use near_sdk::{env, require};

pub mod prelude {
    pub use super::deploy::*;
    pub use super::lazy::Lazy;
    pub use super::owner::*;
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
