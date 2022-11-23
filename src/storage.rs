use near_sdk::{env, require};

use crate::reg;

/// Mesaure cost of storage for a function
pub fn measure_cost<O, F: FnOnce() -> O>(f: F) -> (O, u128) {
    let bytes_used_before = env::storage_usage();
    let res = f();
    let bytes_used = env::storage_usage() - bytes_used_before;
    (res, env::storage_byte_cost() * bytes_used as u128)
}

pub fn left_over_balance<O, F: FnOnce() -> O>(f: F) -> (O, u128) {
    let (res, cost) = measure_cost(f);
    let attached_deposit = near_sdk::env::attached_deposit();
    require!(
        attached_deposit >= cost,
        "Not enough attached deposit to cover storage"
    );
    (res, attached_deposit - cost)
}

/// Excute function f, then transfer funds left over after charging for storage stake to predecessor.
/// ```ignore
/// pub fn contract_method(&mut self, item: Item) {
///   refund_storage_cost(|| {
///     self.push(&item)
///   })
/// }
/// ```
pub fn refund_cost<O, F: FnOnce() -> O>(f: F) -> O {
    let (res, amount_to_refund) = left_over_balance(f);
    if 0 < amount_to_refund {
        let promise_index = reg::promise_batch_create_for_predecessor();
        env::promise_batch_action_transfer(promise_index, amount_to_refund);
    }
    res
}