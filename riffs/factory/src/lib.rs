use near_riffs::{
    near_sdk::{
        self, env, json_types::U128, near_bindgen, require, AccountId, Gas, GasWeight, Promise,
        PromiseResult,
    },
    near_units::{parse_gas, parse_near as near},
    prelude::*,
    reg,
};

pub use near_riffs_core::*;
use near_riffs_registry::Registry;

const INIT_GAS: Gas = Gas(parse_gas!("20 Tgas") as u64);
const MIN_DEPLOY_DEPOSIT: u128 = near!("6 N");

#[near_bindgen]
pub struct Factory {}

#[near_bindgen(riff)]
impl Factory {
    /// Create new account and deploy a contract, and set's the owne to the predecessor_account_id,
    /// e.i. the account that called this contract
    /// 
    /// Requires at least 6N = 6000000000000000000000000
    /// @change
    #[payable]
    pub fn create_subaccount_and_deploy(
        new_account_id: AccountId,
    ) {
        let amount = env::attached_deposit();
        require!(
            amount >= MIN_DEPLOY_DEPOSIT,
            "Requires at least 6N to deploy"
        );
        let current_account_id = env::current_account_id();
        // New name must have no "."'s or be a subaccount of current account
        let trimmed_account = new_account_id
            .as_str()
            .trim_end_matches(&format!(".{current_account_id}"));
        require!(!trimmed_account.contains('.'), "Can only make subaccount");
        let new_account_id = format!("{trimmed_account}.{current_account_id}")
            .parse::<AccountId>()
            .expect("failed to parse account id");

        // Whoever called this contract is the new owner of new_account_id
        let owner_id = env::predecessor_account_id();

        // Create batch promise for sub account
        let promise_index = env::promise_batch_create(&new_account_id);
        // Add create action
        env::promise_batch_action_create_account(promise_index);

        // Transfer attached deposit to subaccount
        env::promise_batch_action_transfer(promise_index, amount);

        // Load the contract's bytes into a register
        let bytes_reg = Registry::get_lazy().as_ref().map_or_else(
            || env::panic_str("Failed to fetch registry"),
            Registry::fetch_to_reg,
        );

        // Use reg module to pass the register instead of byte array
        reg::promise_batch_action_deploy_contract(promise_index, bytes_reg);

        // Initialize contract with at least the bootloader to be owned by owner_id
        env::promise_batch_action_function_call_weight(
            promise_index,
            "set_owner",
            owner_id.as_bytes(),
            0,
            INIT_GAS,
            GasWeight(2),
        );

        // Then attached callback to the current contract
        let final_promise_index = env::promise_batch_then(promise_index, &current_account_id);
        let args =
            format!("{{\"predecessor_account_id\":\"{owner_id}\", \"amount\":\"{amount}\"}}");
        env::promise_batch_action_function_call_weight(
            final_promise_index,
            "on_account_created",
            args.as_bytes(),
            0,
            INIT_GAS,
            GasWeight(1),
        );
        env::promise_return(final_promise_index)
    }

    /// Callback after executing `create_account`.
    #[private]
    pub fn on_account_created(predecessor_account_id: AccountId, amount: U128) -> bool {
        let creation_succeeded = is_promise_success();
        if !creation_succeeded {
            // In case of failure, send funds back.
            Promise::new(predecessor_account_id).transfer(amount.into());
        }
        creation_succeeded
    }
}

fn is_promise_success() -> bool {
    assert_eq!(
        env::promise_results_count(),
        1,
        "Contract expected a result on the callback"
    );
    matches!(env::promise_result(0), PromiseResult::Successful(_))
}
