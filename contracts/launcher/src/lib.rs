use contract_utils::{
    near_sdk::{
        self,
        borsh::{self, BorshDeserialize, BorshSerialize},
        env, ext_contract, near_bindgen, AccountId, Gas, PublicKey, Promise,
    },
    near_units::parse_gas,
    reg,
};

pub use contract_utils::prelude::*;

const GAS: Gas = Gas(parse_gas!("15 TGas") as u64);

#[ext_contract(near)]
trait NearAccount {
    fn create_account(new_account_id: AccountId, new_public_key: PublicKey);
}



#[ext_contract(boot)]
trait Bootloader {
    fn fetch();
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Contract {}

fn network() -> AccountId {
    if cfg!(feature = "testnet") {
        "testnet"
    } else {
        "near"
    }
    .parse()
    .unwrap()
}

fn bootloader() -> AccountId {
    if cfg!(feature = "testnet") {
        "registry.bootloader.testnet"
    } else {
        "registry.bootloader.near"
    }
    .parse()
    .unwrap()
}

#[near_bindgen]
impl Contract {
    ///
    #[payable]
    pub fn launch(&mut self, account_id: AccountId) -> Promise {
        near::ext(network())
            .with_attached_deposit(env::attached_deposit())
            .create_account(account_id.clone(), env::signer_account_pk())
            .then(boot::ext(bootloader()).fetch())
            .then(Contract::ext(env::current_account_id())._deploy_bootloader(account_id))
    }

    #[private]
    pub fn _deploy_bootloader(&mut self, account_id: AccountId) {
        let id = env::promise_batch_create(&account_id);
        reg::promise_batch_action_deploy_contract(id, reg::promise_result(0, 0));
        env::promise_batch_action_function_call(
            id,
            "set_owner",
            env::predecessor_account_id().as_bytes(),
            0,
            GAS,
        );
        env::promise_batch_action_delete_key(id, &env::signer_account_pk())
    }
}
