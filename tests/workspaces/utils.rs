use anyhow::{Context, Result};
use near_units::parse_near;
use workspaces::network::Sandbox;
use workspaces::result::ExecutionFinalResult;
use workspaces::{Account, AccountId, Contract, DevNetwork, Worker};

lazy_static_include::lazy_static_include_bytes! {
  pub BOOTLOADER => "./target/res/bootloader.wasm",
  pub REGISTRY => "./target/res/contract_registry.wasm",
  pub STATUS_MESSAGE => "./target/res/status_message.wasm",
  pub STATUS_MESSAGE_BINDGEN => "./target/res/status_message_bindgen.wasm",
  pub CONTRACT_LAUNCHER => "./target/res/contract_launcher.wasm",
  pub NEAR_WASM => "./target/res/near.wasm"

}

pub struct TestEnv {
    pub root: Account,
    pub worker: Worker<Sandbox>,
    pub alice: Account,
    pub bob: Account,
}

pub trait IntoVec {
    fn to_vec(&self) -> Vec<u8>;
}

impl IntoVec for Account {
    fn to_vec(&self) -> Vec<u8> {
        self.id().as_bytes().to_vec()
    }
}

pub trait AssertResult {
    fn assert_success(self) -> Self;
    fn assert_failure(self) -> Self;
}

impl AssertResult for ExecutionFinalResult {
    fn assert_success(self) -> Self {
        let name = &self.outcome().executor_id;
        assert!(
            self.is_success(),
            "Transaction from {name} failed with:\n{:#?}",
            self
        );
        self
    }

    fn assert_failure(self) -> Self {
        let name = &self.outcome().executor_id;
        assert!(
            self.is_failure(),
            "Transaction from {name} should have failed"
        );
        self
    }
}

impl TestEnv {
    pub async fn init() -> Result<Self> {
        let worker = workspaces::sandbox().await?;
        let root = worker.root_account()?;
        let alice = root
            .create_subaccount("alice")
            .transact()
            .await?
            .into_result()?;
        let bob = root
            .create_subaccount("bob")
            .transact()
            .await?
            .into_result()?;

        Ok(Self {
            worker,
            root,
            alice,
            bob,
        })
    }

    pub async fn with_bootloader() -> Result<(Contract, Self)> {
        let this = Self::init().await?;
        let bootloader = this.bootloader().await?;
        Ok((bootloader, this))
    }

    pub async fn with_lancher() -> Result<(Contract, Self)> {
        let this = Self::init().await?;
        let launcher = this.launcher().await?;
        Ok((launcher, this))
    }


    pub async fn launcher(&self) -> Result<Contract, workspaces::error::Error> {
        self.worker.dev_deploy(&CONTRACT_LAUNCHER).await
    }

    pub async fn bootloader(&self) -> Result<Contract> {
      deploy_and_init(&self.worker, &BOOTLOADER, &self.root).await
    }

    pub async fn registry(&self) -> Result<Contract> {
        deploy_and_init(&self.worker, &REGISTRY, &self.root).await
    }

    pub async fn registry_with_bootlader(&self) -> Result<Contract> {
        let reg = self.registry().await?;
        assert!(self.patch_bootlader(reg.id()).await?.is_success());
        Ok(reg)
    }

    pub async fn patch(
        &self,
        contract_id: &AccountId,
        bytes: Vec<u8>,
    ) -> Result<ExecutionFinalResult, workspaces::error::Error> {
        self.root
            .call(contract_id, "patch")
            .args(bytes)
            .gas(300_000_000_000_000)
            .deposit(parse_near!("1 N"))
            .transact()
            .await
    }

    pub async fn patch_bootlader(
        &self,
        contract_id: &AccountId,
    ) -> Result<ExecutionFinalResult, workspaces::error::Error> {
        self.patch(contract_id, BOOTLOADER.to_vec()).await
    }

    pub async fn deploy_root_contract(&self) -> Result<Contract> {
        Ok(self.root.deploy(&NEAR_WASM).await?.result)
    }
}


pub async fn deploy_and_init(
    worker: &Worker<impl DevNetwork>,
    bytes: &[u8],
    owner: &Account,
) -> Result<Contract> {
    let contract = worker.dev_deploy(bytes).await?;
    let res = contract
        .call("set_owner")
        .args(format!("\"{}\"", owner.id()).as_bytes().to_vec())
        .transact()
        .await
        .with_context(|| format!("Owner that failed: {}", owner.id()))?;
    println!("Created Contract: {:?}", res.logs());
    Ok(contract)
}
