use crate::account::{self, assert_private};
use crate::owner::Owner;
use near_units::parse_gas;

pub mod promise;
use near_sdk::{env, sys, Gas, AccountId};
use promise::*;

const FETCH_GAS: u64 = parse_gas!("70 TGas") as u64;
const DEPLOY_GAS: u64 = parse_gas!("70 TGas") as u64;

#[no_mangle]
pub fn deploy() {
    Owner::assert();
    let (arguments, account_id) = parse_input();
    let id = env::promise_create(account_id, "fetch", &arguments, 0, Gas(FETCH_GAS));
    env::promise_return(account::promise_then_for_current(
        0,
        id,
        "_deploy",
        &[],
        0,
        DEPLOY_GAS,
    ))
}

#[no_mangle]
pub fn _deploy() {
    assert_private();

    unsafe {
        sys::promise_return(cheap_deploy(promise_result()));
    }
}

fn parse_input() -> (Vec<u8>, AccountId) {
      // v0_0_1.tenk.near
      // Currently checking string adds 10K to contract
      let input_account_id: String = unsafe { String::from_utf8_unchecked(env::input().unwrap()) };
      let (version, subaccount) = input_account_id.as_str().split_once('.').unwrap();
      let arguments = version.strip_prefix('v').unwrap_or(version).as_bytes().to_vec();
      (arguments, format!("registry.{subaccount}").parse().unwrap())

}
