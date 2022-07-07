use crate::{AssertOwnable, Ownable};
use near_components::{
    account::assert_private,
    account_id_from_input,
    near_sdk::{env, AccountId},
    near_units::parse_gas,
    promise, reg,
};

const FETCH_GAS: u64 = parse_gas!("70 TGas") as u64;
const DEPLOY_GAS: u64 = parse_gas!("70 TGas") as u64;

struct Deployer;

impl Ownable for Deployer {}

//#[default(Deployer)]
pub trait Deployable {
    /// Deploy a contract from a passed registry
    fn deploy() {
      Deployer::deploy()
    }

    fn _deploy() {
      Deployer::_deploy()
    }
}

impl Deployer {
    fn deploy() {
        Self::assert_owner();
        let (arguments, account_id) = parse_input();
        let id = promise::promise_create(account_id.as_str(), "fetch", &arguments, 0, FETCH_GAS);
        env::promise_return(reg::promise_then_for_current(
            id,
            "_deploy",
            &[],
            0,
            DEPLOY_GAS,
        ))
    }

    fn _deploy() {
        assert_private();
        let bytes_reg = reg::promise_result(0, 0);
        env::promise_return(reg::promise_batch_action_deploy_contract_for_current(
            bytes_reg,
        ))
    }
}

// #[no_mangle]
// pub fn deploy() {}

// #[no_mangle]
// pub fn _deploy() {}

fn parse_input() -> (Vec<u8>, AccountId) {
    // v0_0_1.tenk.near
    // Currently checking string adds 10K to contract
    let input_account_id: String = account_id_from_input().into();
    let (version, subaccount) = input_account_id.as_str().split_once('.').unwrap();
    let arguments = version
        .strip_prefix('v')
        .unwrap_or(version)
        .as_bytes()
        .to_vec();
    (arguments, format!("registry.{subaccount}").parse().unwrap())
}
