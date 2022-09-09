use near_sdk::require;

use crate::{near_sdk::sys, reg};

pub type FixedAccountId = [u8; 64];

pub fn assert_private() {
    let current_account_id = current_account_id();
    let predecessor_account_id = predecessor_account_id();
    require!(current_account_id == predecessor_account_id, "");
}

pub fn predecessor_account_id() -> FixedAccountId {
    read_register(reg::predecessor_account_id())
}

pub fn current_account_id() -> FixedAccountId {
    read_register(reg::current_account_id())
}

pub fn input_account_id() -> FixedAccountId {
    read_register(reg::input())
}

pub fn read_register(register_id: u64) -> FixedAccountId {
    let mut res = [0u8; 64];
    unsafe {
        sys::read_register(register_id, res.as_mut_ptr() as _);
    }
    res
}

// pub fn create_promise_for_predecessor(register_id: u64) -> u64 {
//     unsafe {
//         sys::predecessor_account_id(register_id);
//     };
//     reg::promise_batch_create(register_id)
// }

// pub fn promise_batch_create_for_current(register_id: u64) -> u64 {
//     unsafe {
//         sys::current_account_id(register_id);
//     };
//     reg::promise_batch_create(register_id)
// }
