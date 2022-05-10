use near_sdk::{env, sys, Balance};

use crate::account;

pub fn promise_batch_then(promise_index: u64, account_id: &[u8; 64]) -> u64 {
    unsafe { sys::promise_batch_then(promise_index, 64, account_id.as_ptr() as _) }
}

pub fn promise_batch_create(account_id: &[u8; 64]) -> u64 {
    unsafe { sys::promise_batch_create(account_id.len() as _, account_id.as_ptr() as _) }
}


pub fn promise_batch_action_function_call(
    promise_index: u64,
    method_name: &str,
    arguments: &[u8],
    amount: Balance,
    gas: u64,
) {
    unsafe {
        sys::promise_batch_action_function_call(
            promise_index,
            method_name.len() as _,
            method_name.as_ptr() as _,
            arguments.len() as _,
            arguments.as_ptr() as _,
            &amount as *const Balance as _,
            gas,
        )
    }
}


pub fn promise_batch_action_function_call_fetch(promise_index: u64, function_name: &str, gas: u64) {
    let amount = 0u128;

    unsafe {
        sys::input(3);
        sys::promise_batch_action_function_call(
            promise_index,
            function_name.len() as _,
            function_name.as_ptr() as _,
            u64::MAX,
            3u64,
            &amount as *const Balance as _,
            gas,
        )
    }
}

pub fn cheap_deploy(register: u64) -> u64 {
    let id = account::create_promise_for_current(register);
    unsafe {
        sys::promise_batch_action_deploy_contract(id, u64::MAX, register);
    }
    id
}

pub fn promise_result() -> u64 {
    match unsafe { sys::promise_result(0, 1) } {
        1 => 1,
        _ => env::panic_str("promise failed"),
    }
}
