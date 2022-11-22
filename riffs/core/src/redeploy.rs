use crate::Owner;
use near_riffs::{
    account::assert_private,
    input,
    near_sdk::{self, env, near_bindgen, AccountId, Gas},
    near_units::parse_gas,
    prelude::Lazy,
    reg,
};

const FETCH_GAS: u64 = parse_gas!("70 Tgas") as u64;
const DEPLOY_GAS: u64 = parse_gas!("70 Tgas") as u64;

#[derive(Default)]
#[near_bindgen]
pub struct Redeployer;

impl Lazy for Redeployer {
    fn get_lazy() -> Option<Self> {
        Some(Redeployer {})
    }

    fn set_lazy(_: Self) -> Option<Self> {
        None
    }
}

impl Redeployer {
    pub fn redeploy() {
        Owner::assert_with_one_yocto();
        let (arguments, account_id) = parse_input();
        Self::redeploy_account(account_id, &arguments);
    }

    pub fn on_redeploy() {
        assert_private();
        let promise_value_reg = reg::promise_result(0);
        env::promise_return(reg::promise_batch_action_deploy_contract_for_current(
            promise_value_reg,
        ))
    }
}

#[no_mangle]
pub fn redeploy() {
    Redeployer::redeploy()
}

#[no_mangle]
pub fn on_redeploy() {
    Redeployer::on_redeploy()
}

impl Redeployer {
    pub fn redeploy_account(account_id: AccountId, arguments: &[u8]) {
        let id = env::promise_create(account_id, "fetch", arguments, 0, Gas(FETCH_GAS));
        env::promise_return(reg::promise_then_for_current(
            id,
            "on_redeploy",
            &[],
            0,
            DEPLOY_GAS,
        ))
    }
}

fn parse_input() -> (Vec<u8>, AccountId) {
    // v0_0_1.tenk.near
    // Currently checking string adds 10K to contract
    let input_account_id: String = input::account_id().into();
    let (version, subaccount) = input_account_id.as_str().split_once('.').unwrap();
    let arguments = version
        .strip_prefix('v')
        .unwrap_or(version)
        .as_bytes()
        .to_vec();
    (arguments, subaccount.parse().unwrap())
}

#[allow(dead_code, unused_variables)]
mod private {
    use near_riffs::{near_sdk::AccountId, witgen};

    /// Redeploys contract from  provided version and registry.
    /// e.g. `v0_0_1.contract.testnet`
    /// @change
    #[witgen]
    pub fn redeploy(account_id: AccountId) {}
}
