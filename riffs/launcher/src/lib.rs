use near_riffs::{
    near_sdk::{
        self,
        borsh::{self, BorshDeserialize, BorshSerialize},
        env, ext_contract,
        json_types::U128,
        near_bindgen, require,
        serde::Serialize,
        AccountId, Gas, GasWeight, Promise, PromiseResult, PublicKey,
    },
    near_units::{parse_gas, parse_near as near},
    prelude::*,
    reg, witgen,
};

pub use near_riffs_core::*;
use near_riffs_registry::Registry;

const INIT_GAS: Gas = Gas(parse_gas!("20 Tgas") as u64);
const MIN_DEPLOY_DEPOSIT: u128 = near!("6 N");

#[near_bindgen]
pub struct Launcher {}

impl Default for Launcher {
    fn default() -> Self {
        Self {}
    }
}

impl Lazy for Launcher {
    fn get_lazy() -> Option<Self> {
        Some(Self {})
    }

    fn set_lazy(value: Self) -> Option<Self> {
        Some(value)
    }
}

#[near_bindgen(riff)]
impl Launcher {
    /// Create new account without linkdrop and deposit passed funds (used for creating sub accounts directly).
    /// Then Deploy a contract and optionally call an init method
    #[payable]
    pub fn create_subaccount_and_deploy(
        &mut self,
        new_account_id: AccountId,
        new_public_key: PublicKey,
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
        env::log_str(&format!("new_account_id, {}", new_account_id));

        // Whoever called this contract is the new owner of new_account_id
        let owner_id = env::predecessor_account_id();

        // create, deploy, and initialize the new account
        let promise_index = env::promise_batch_create(&new_account_id);
        env::promise_batch_action_create_account(promise_index);
        env::log_str(&format!("Create Account"));

        env::promise_batch_action_add_key_with_full_access(promise_index, &new_public_key, 0);
        env::promise_batch_action_transfer(promise_index, amount);
        
        // Load the contract's bytes into a register
        let registry = Registry::get_lazy();
        let bytes_reg = if let Some(registry) = registry {
            env::log_str(&format!("About to get registry"));
            registry.fetch_to_reg()
        } else {
            env::panic_str(&format!("Failed to fetch registry"));
        };

        env::log_str(&format!("fetched contract bytes"));

        reg::promise_batch_action_deploy_contract(promise_index, bytes_reg);
        env::promise_batch_action_function_call_weight(
            promise_index,
            "set_owner",
            owner_id.as_bytes(),
            0,
            INIT_GAS,
            GasWeight(2),
        );
        env::log_str(&format!("First Promise done"));

        // Then attached callback to the current contract
        let final_promise_index = env::promise_batch_then(promise_index, &current_account_id);
        let args =
            format!("{{\"predecessor_account_id\":\"{owner_id}\", \"amount\":\"{amount}\"}}");
        env::log_str(&args);
        env::promise_batch_action_function_call_weight(
            final_promise_index,
            "on_account_created",
            args.as_bytes(),
            0,
            INIT_GAS,
            GasWeight(1),
        );
        env::log_str(&format!("Done"));
        env::promise_return(final_promise_index)
    }

    /// Callback after executing `create_account`.
    pub fn on_account_created(&mut self, predecessor_account_id: AccountId, amount: U128) -> bool {
        assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "Callback can only be called from the contract"
        );
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
    match env::promise_result(0) {
        PromiseResult::Successful(_) => true,
        _ => false,
    }
}
