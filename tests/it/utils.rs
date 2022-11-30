use anyhow::Result;
use near_units::parse_near as N;
use near_units::parse_near as near;
use serde_json::json;
use workspaces::network::Sandbox;
use workspaces::operations::Function;
use workspaces::result::ExecutionFinalResult;
use workspaces::{
    types::{KeyType, PublicKey, SecretKey},
    Account, AccountId, Contract, Worker,
};

pub const SIX_NEAR: u128 = near!("6 N");

lazy_static_include::lazy_static_include_bytes! {
  pub BOOTLOADER => "./target/res/bootloader.wasm",
  pub REGISTRY => "./target/res/registry.wasm",
  pub STATUS_MESSAGE => "./target/res/status_message.wasm",
  pub STATUS_MESSAGE_BINDGEN => "./target/res/status_message_bindgen.wasm",
  pub LAUNCHER => "./target/res/contract_launcher.wasm",
  pub NEAR_WASM => "./target/res/near.wasm",
  pub FACTORY => "./target/res/factory.wasm",
  pub BOOTLOADER_LOCKED => "./target/res/bootloader_locked.wasm",

}

pub enum Contracts {
    Bootloader,
    BootloaderLocked,
    Registry,
    Factory,
    NearRoot,
    Launcher,
}

impl From<Contracts> for Vec<u8> {
    fn from(value: Contracts) -> Vec<u8> {
        match value {
            Contracts::Bootloader => BOOTLOADER.to_vec(),
            Contracts::BootloaderLocked => BOOTLOADER_LOCKED.to_vec(),
            Contracts::Registry => REGISTRY.to_vec(),
            Contracts::Factory => FACTORY.to_vec(),
            Contracts::NearRoot => NEAR_WASM.to_vec(),
            Contracts::Launcher => LAUNCHER.to_vec(),
        }
    }
}

pub const ALICE: &str = "alice";
#[allow(dead_code)]
pub const BOB: &str = "bob";

pub type WsResult<T> = Result<T, workspaces::error::Error>;

pub struct TestEnv {
    pub root: Account,
    pub worker: Worker<Sandbox>,
}

pub trait IntoVec {
    fn to_vec(&self) -> Vec<u8>;
}

impl IntoVec for Account {
    fn to_vec(&self) -> Vec<u8> {
        self.id().as_bytes().to_vec()
    }
}

impl IntoVec for str {
    fn to_vec(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

pub trait AssertResult {
    fn assert_success(self);
    fn assert_failure(self);
}

impl AssertResult for ExecutionFinalResult {
    fn assert_success(self) {
        let name = &self.outcome().executor_id;
        assert!(
            self.is_success(),
            "Transaction from {name} failed with:\n{:#?}",
            self
        );
    }

    fn assert_failure(self) {
        let name = &self.outcome().executor_id;
        assert!(
            self.is_failure(),
            "Transaction from {name} should have failed:\n{self:#?}",
        );
    }
}

fn first<T0, T1>(t: (T0, T1)) -> T0 {
    t.0
}

fn second<T0, T1>(t: (T0, T1)) -> T1 {
    t.1
}

fn to_account_id(s: &str) -> Option<AccountId> {
    s.parse().ok()
}

pub trait AccountIdTools {
    /// parent account until root account which repeats
    fn parent(&self) -> AccountId;

    fn first_account(&self) -> AccountId;

    /// Given a new subaccount without any "."s
    fn subaccount(&self, name: &str) -> AccountId;

    /// SecretKey generated using AccountId as seed.
    /// 
    /// Currently ED25519 only
    fn to_sk(&self) -> SecretKey;

    /// # PublicKey generated using AccountId as seed.
    /// 
    /// Currently ED25519 only
    fn to_pk(&self) -> PublicKey;
}

impl AccountIdTools for AccountId {
    fn parent(&self) -> AccountId {
        self.split_once('.')
            .map(second)
            .and_then(to_account_id)
            .unwrap_or_else(|| self.clone())
    }

    fn first_account(&self) -> AccountId {
        self.split_once('.')
            .map(first)
            .and_then(to_account_id)
            .unwrap_or_else(|| self.clone())
    }

    fn subaccount(&self, name: &str) -> AccountId {
        format!("{name}.{self}").parse().unwrap()
    }

    fn to_sk(&self) -> SecretKey {
        ed25519::secret_key_from_seed(self.as_str())
    }

    fn to_pk(&self) -> PublicKey {
        ed25519::secret_key_from_seed(self.as_str()).public_key()
    }
}

impl TestEnv {
    pub async fn init() -> Result<Self> {
        let worker = workspaces::sandbox().await?;
        let root = worker.root_account()?;
        Ok(Self { worker, root })
    }

    pub async fn redeploy(
        &self,
        owner: &Account,
        contract: &Contract,
        registry: &Contract,
    ) -> WsResult<ExecutionFinalResult> {
        let version = registry
            .view("current_version", vec![])
            .await?
            .json::<String>()?;
        owner
            .call(contract.id(), "redeploy")
            .args(format!("{}.{}", version, registry.id()).into_bytes())
            .deposit(1)
            .max_gas()
            .transact()
            .await
    }

    pub async fn deploy_and_init_subaccount(
        &self,
        bytes: &[u8],
        new_account_id: &AccountId,
        root: &Account,
    ) -> WsResult<Contract> {
        let parent = new_account_id.parent();
        assert_eq!(parent.as_str(), root.id().as_str());
        root.batch(new_account_id)
            .create_account()
            .transfer(N!("6 N"))
            .deploy(bytes)
            .call(Function::new("set_owner").args(format!("\"{}\"", parent).as_bytes().to_vec()))
            .transact()
            .await?
            .assert_success();
        let contract =
            Contract::from_secret_key(new_account_id.clone(), new_account_id.to_sk(), &self.worker);

        Ok(contract)
    }

    pub async fn with_bootloader() -> Result<(Contract, Self)> {
        let this = Self::init().await?;
        let bootloader = this.bootloader().await?;
        Ok((bootloader, this))
    }

    pub async fn with_launcher() -> Result<(Contract, Self)> {
        let this = Self::init().await?;
        let launcher = this.launcher().await?;
        Ok((launcher, this))
    }

    pub async fn launcher(&self) -> WsResult<Contract> {
        self.worker.dev_deploy(&LAUNCHER).await
    }

    pub async fn bootloader(&self) -> WsResult<Contract> {
        self.deploy_and_init_subaccount(
            &BOOTLOADER,
            &self.root.id().subaccount("bootloader"),
            &self.root,
        )
        .await
    }

    pub async fn registry(&self, initial_contract: Contracts) -> WsResult<Contract> {
        let contract = self
            .deploy_and_init_subaccount(
                &REGISTRY,
                &self.root.id().subaccount("registry"),
                &self.root,
            )
            .await?;
        self.patch(contract.id(), initial_contract.into())
            .await?
            .assert_success();
        Ok(contract)
    }

    pub async fn factory(&self, new_account_id: &str, contract: Contracts) -> Result<Contract> {
        let factory = self
            .deploy_and_init_subaccount(
                &FACTORY,
                &self.root.id().subaccount(new_account_id),
                &self.root,
            )
            .await?;
        self.patch(factory.id(), contract.into())
            .await?
            .assert_success();
        Ok(factory)
    }

    pub async fn patch(
        &self,
        contract_id: &AccountId,
        bytes: Vec<u8>,
    ) -> WsResult<ExecutionFinalResult> {
        self.root
            .call(contract_id, "patch")
            .args(bytes)
            .gas(300_000_000_000_000)
            .deposit(N!("5 N"))
            .transact()
            .await
    }

    pub async fn deploy_root_contract(&self) -> Result<Contract> {
        Ok(self.root.deploy(&NEAR_WASM).await?.result)
    }

    pub async fn create_subaccount(&self, name: &str) -> Result<Account> {
        Ok(self
            .root
            .create_subaccount(name)
            .transact()
            .await?
            .into_result()?)
    }

    pub async fn alice(&self) -> Result<Account> {
        self.create_subaccount(ALICE).await
    }

    pub async fn create_subaccount_and_deploy(
        &self,
        factory: &Contract,
        new_account_id: &str,
    ) -> anyhow::Result<Contract> {
        let new_account_id = factory.id().subaccount(new_account_id);
        let secret_key = new_account_id.to_sk();
        let txn = self
            .root
            .call(factory.id(), "create_subaccount_and_deploy")
            .args_json(json!({
                "new_account_id": new_account_id,
                "new_public_key": secret_key.public_key(),
            }))
            .deposit(SIX_NEAR)
            .max_gas()
            .transact()
            .await?;
        txn.assert_success();

        Ok(Contract::from_secret_key(
            new_account_id,
            secret_key,
            &self.worker,
        ))
    }
}

// pub async fn deploy_and_init(
//     worker: &Worker<impl DevNetwork>,
//     bytes: &[u8],
//     owner: &Account,
// ) -> Result<Contract> {
//     let contract = worker.dev_deploy(bytes).await?;
//     let res = contract
//         .call("set_owner")
//         .args(format!("\"{}\"", owner.id()).as_bytes().to_vec())
//         .transact()
//         .await
//         .with_context(|| format!("Owner that failed: {}", owner.id())).and_then(op)
//     println!("Created Contract: {:?}", res.logs());
//     Ok(contract)
// }

// pub const fn is_ok<U, F, G: FnOnce(T) -> Result<U, E>>(self, f: F, ) -> Option<U>
// where
//     F: ~const FnOnce(T) -> Option<U>,
//     F: ~const Destruct,
// {

pub mod ed25519 {
    use super::{KeyType, PublicKey, SecretKey};

    #[allow(dead_code)]
    pub fn public_key_from_seed(seed: &str) -> PublicKey {
        SecretKey::from_seed(KeyType::ED25519, seed).public_key()
    }

    pub fn secret_key_from_seed(seed: &str) -> SecretKey {
        SecretKey::from_seed(KeyType::ED25519, seed)
    }
}
