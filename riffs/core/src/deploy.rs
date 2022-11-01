use crate::Owner;
use near_riffs::{
    account::assert_private,
    account_id_from_input,
    near_sdk::{self, env, near_bindgen, AccountId},
    near_units::parse_gas,
    prelude::Lazy,
    promise, reg,
};

const FETCH_GAS: u64 = parse_gas!("70 Tgas") as u64;
const DEPLOY_GAS: u64 = parse_gas!("70 Tgas") as u64;

#[derive(Default)]
#[near_bindgen]
pub struct Deployer;

impl Lazy for Deployer {
    fn get_lazy() -> Option<Self> {
        Some(Deployer {})
    }

    fn set_lazy(_: Self) -> Option<Self> {
        None
    }
}

// #[default(Deployer)]
// pub trait Deployable {
//     /// Deploy a contract from a passed registry
//     fn deploy() {
//         Deployer::deploy()
//     }

//     fn _deploy() {
//         Deployer::_deploy()
//     }
// }

#[near_bindgen(riff)]
impl Deployer {
    pub fn deploy(&self) {
        Owner::assert_owner();
        let (arguments, account_id) = parse_input();
        Self::deploy_account(account_id, &arguments);
    }

    pub fn _deploy(&self) {
        assert_private();
        let promise_value_reg = reg::promise_result(0);
        env::promise_return(reg::promise_batch_action_deploy_contract_for_current(
            promise_value_reg,
        ))
    }
}

impl Deployer {
    pub fn deploy_account(account_id: AccountId, arguments: &[u8]) {
        let id = promise::promise_create(account_id.as_str(), "fetch", &arguments, 0, FETCH_GAS);
        env::promise_return(reg::promise_then_for_current(
            id,
            "_deploy",
            &[],
            0,
            DEPLOY_GAS,
        ))
    }
}

// #[no_mangle]
// pub fn deploy() {
//     Deployer::default().deploy();
// }

// #[no_mangle]
// pub fn _deploy() {
//     Deployer::default()._deploy();
// }

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
    (arguments, subaccount.parse().unwrap())
}

// #[allow(dead_code, unused_variables)]
// mod private {

//   use near_riffs::{near_sdk::AccountId, witgen};

//   #[witgen]
//   /// Redeploys contract from  provided registry.
//   /// e.g. `v0_0_1.contract.testnet`
//   /// @change
//   pub fn deploy(account_id: AccountId) {}

// }
