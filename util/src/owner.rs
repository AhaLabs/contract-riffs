use std::mem::MaybeUninit;

use crate::{account, reg};
use near_sdk::{env, require};

const EVICTED_REGISTER: u64 = std::u64::MAX - 1;
pub const OWNER_KEY: &str = "OWNER";

pub fn get_owner() -> Option<[u8; 64]> {
    let hash = [MaybeUninit::<u8>::uninit(); 64];
    reg::storage_read(0, EVICTED_REGISTER)?;
    unsafe { Some(std::mem::transmute(hash)) }
}

#[no_mangle]
pub fn set_owner() -> bool {
    _set_owner(&account::input_account_id());
    true
}

pub fn _set_owner(owner_id: &[u8; 64]) {
    if env::storage_write(OWNER_KEY.as_bytes(), owner_id) {
        require!(
            account::read_register(EVICTED_REGISTER) == account::predecessor_account_id(),
            "",
        )
    }
}

pub fn assert_private() {
  let current_account_id = account::current_account_id();
  let predecessor_account_id = account::predecessor_account_id();
  require!(current_account_id == predecessor_account_id, "");
}

pub fn assert_owner() {
  let current_account_id = get_owner();
  require!(current_account_id.is_some(), "");
  let predecessor_account_id = account::predecessor_account_id();
  require!(current_account_id.unwrap() == predecessor_account_id, "");
}