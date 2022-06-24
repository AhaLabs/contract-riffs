use contract_utils::{
    near_sdk::{
        self,
        borsh::{self, BorshDeserialize, BorshSerialize},
        env, ext_contract, near_bindgen,
        serde::Serialize,
        AccountId, Gas, Promise, PublicKey,
    },
    near_units::parse_gas,
    reg, witgen,
};

pub use contract_utils::prelude::*;

const GAS: Gas = Gas(parse_gas!("15 TGas") as u64);

#[ext_contract(near)]
trait NearAccount {
    fn create_account_and_deploy(
        new_account_id: AccountId,
        new_public_key: PublicKey,
        bytes: Vec<u8>,
        init_method: Option<String>,
        args: Option<Vec<u8>>,
    );
}

#[ext_contract(boot)]
trait Bootloader {
    fn fetch();
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[witgen]
pub struct Contract {
    /// default account that the new contract will be a sub account of
    root_account: AccountId,
    /// default registry that will return the contract to be deployed
    registry: AccountId,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            root_account: root_account(),
            registry: registry(),
        }
    }
}

fn root_account() -> AccountId {
    if cfg!(feature = "testnet") {
        "tn"
    } else {
        "near"
    }
    .parse()
    .unwrap()
}

fn registry() -> AccountId {
    if cfg!(feature = "testnet") {
        "registry.bootloader.tn"
    } else {
        "registry.bootloader.near"
    }
    .parse()
    .unwrap()
}

#[near_bindgen]
impl Contract {
    pub fn update(&mut self, root_account: Option<AccountId>, registry: Option<AccountId>) {
        self.assert_owner();
        if let Some(root_account) = root_account {
            self.root_account = root_account;
        }
        if let Some(registry) = registry {
            self.registry = registry;
        }
    }

    /// Create account and deploy a contract from a registry, bootloader contract by default
    #[payable]
    pub fn launch(
        &mut self,
        account_id: AccountId,
        registry: Option<AccountId>,
        root_account: Option<AccountId>,
    ) -> Promise {
        boot::ext(registry.unwrap_or_else(|| self.registry.clone()))
            .fetch()
            .then(
                Self::ext(env::current_account_id())
                    .with_attached_deposit(env::attached_deposit())
                    ._fetch(
                        account_id,
                        env::predecessor_account_id(),
                        root_account.unwrap_or_else(|| self.root_account.clone()),
                    ),
            )
    }

    #[private]
    pub fn _fetch(&self, account_id: AccountId, owner: AccountId, root_account: AccountId) {
        let amount = env::attached_deposit();
        env::log_str(&amount.to_string());
        let bytes = match env::promise_result(0) {
            near_sdk::PromiseResult::Successful(data) => data,
            _ => env::panic_str("failed to fetch bytes"),
        };
        near::ext(root_account)
            .with_attached_deposit(amount)
            .create_account_and_deploy(
                account_id.clone(),
                env::signer_account_pk(),
                bytes,
                Some("set_owner".to_string()),
                Some(owner.as_bytes().to_vec()),
            );
    }

    pub fn accounts(self) -> Contract {
        self
    }
}
