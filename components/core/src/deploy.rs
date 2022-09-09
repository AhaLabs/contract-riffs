use crate::Owner;
use near_components::{
    account::assert_private,
    account_id_from_input,
    near_sdk::{env, AccountId},
    near_units::parse_gas,
    prelude::Lazy,
    promise, reg,
};

const FETCH_GAS: u64 = parse_gas!("70 TGas") as u64;
const DEPLOY_GAS: u64 = parse_gas!("70 TGas") as u64;

#[derive(Default)]
pub struct Deployer;

impl Lazy for Deployer {
    fn get_lazy() -> Option<Self> {
        Some(Deployer {})
    }

    fn set_lazy(_: Self) -> Option<Self> {
        None
    }
}

//#[default(Deployer)]
// pub trait Deployable {
//     /// Deploy a contract from a passed registry
//     fn deploy() {
//         Deployer::deploy()
//     }

//     fn _deploy() {
//         Deployer::_deploy()
//     }
// }

impl Deployer {
    pub fn deploy() {
        Owner::assert_owner();
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

    pub fn _deploy() {
        assert_private();
        let bytes_reg = reg::promise_result(0, 0);
        env::promise_return(reg::promise_batch_action_deploy_contract_for_current(
            bytes_reg,
        ))
    }
}

#[no_mangle]
pub fn deploy() {
    Deployer::deploy();
}

#[no_mangle]
pub fn _deploy() {
    Deployer::_deploy();
}

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

//   use near_components::{near_sdk::AccountId, witgen};

//   #[witgen]
//   /// Redeploys contract from  provided registry.
//   /// e.g. `v0_0_1.contract.testnet`
//   /// @change
//   pub fn deploy(account_id: AccountId) {}

// }
