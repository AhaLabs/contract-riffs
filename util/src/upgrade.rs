use near_sdk::{env, sys, Balance};

use crate::account::{self, assert_private};
use crate::owner::*;

const FETCH_GAS: u64 = 70000000000000;
const DEPLOY_GAS: u64 = 70000000000000;

trait Deployable: Ownable {
    fn deploy(&self) {
        self.assert_predecessor();
        let current_account_id = account::current_account_id();
        let id = promise_batch_create(&self.registry());
        promise_batch_action_function_call_fetch(id, "fetch_binary", FETCH_GAS);
        let self_id = promise_batch_then(id, &current_account_id);
        promise_batch_action_function_call(self_id, "_deploy", &[], 0, DEPLOY_GAS);
    }

    fn registry(&self) -> [u8; 64] {
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
}

fn promise_batch_then(promise_index: u64, account_id: &[u8; 64]) -> u64 {
    unsafe { sys::promise_batch_then(promise_index, 64, account_id.as_ptr() as _) }
}

fn promise_batch_create(account_id: &[u8; 64]) -> u64 {
    unsafe { sys::promise_batch_create(account_id.len() as _, account_id.as_ptr() as _) }
}

#[no_mangle]
pub fn deploy() {
    Deployable::deploy(&Deployer {})
}

// #[derive(BorshDeserialize, BorshSerialize,)]
struct Deployer {}

// impl IntoStorageKey for Deployer {
//   fn into_storage_key(self) -> Vec<u8> {
//       OWNER_KEY.as_bytes().to_vec()
//   }
// }

impl Deployable for Deployer {}

fn promise_batch_action_function_call(
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

pub fn _deploy() {
    assert_private();
    unsafe {
        sys::promise_return(cheap_deploy(promise_result()));
    }
}

fn promise_batch_action_function_call_fetch(promise_index: u64, function_name: &str, gas: u64) {
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

fn cheap_deploy(register: u64) -> u64 {
    let id = account::create_promise_for_current(register);
    unsafe {
        sys::promise_batch_action_deploy_contract(id, u64::MAX, register);
    }
    id
}

fn promise_result() -> u64 {
    match unsafe { sys::promise_result(0, 1) } {
        1 => 1,
        _ => env::panic_str("promise failed"),
    }
}
