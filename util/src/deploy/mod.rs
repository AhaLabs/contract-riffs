use crate::account::{self, assert_private};
use crate::owner::Owner;

mod promise;
use near_sdk::sys;
use promise::*;

const FETCH_GAS: u64 = 70000000000000;
const DEPLOY_GAS: u64 = 70000000000000;

fn registry() -> [u8; 64] {
    let mut res = [0u8; 64];
    let name = if cfg!(feature = "testnet") {
        "contractregistry.testnet"
    } else {
        "contractregistry.near"
    };
    let b: &[u8] = name.as_bytes();
    res.copy_from_slice(b);
    res
}
#[no_mangle]
pub fn deploy() {
  Owner::assert();
  let current_account_id = account::current_account_id();
  let id = promise_batch_create(&registry());
  promise_batch_action_function_call_fetch(id, "fetch_binary", FETCH_GAS);
  let self_id = promise_batch_then(id, &current_account_id);
  promise_batch_action_function_call(self_id, "_deploy", &[], 0, DEPLOY_GAS);
}

#[no_mangle]
pub fn _deploy() {
    assert_private();
    unsafe {
        sys::promise_return(cheap_deploy(promise_result()));
    }
}