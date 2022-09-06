use anyhow::{Context, Result};
use near_units::{parse_near};
use serde_json::{json, Value};
use workspaces::network::Sandbox;
use workspaces::operations::Function;
use workspaces::result::{ExecutionFinalResult};
use workspaces::{Account, Contract, DevNetwork, Worker};

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
    worker: Worker<Sandbox>,
    pub alice: Account,
    pub bob: Account,
    bootloader: Option<Contract>,
    registry: Option<Contract>,
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
            bootloader: None,
            registry: None,
        })
    }

    pub async fn with_bootloader() -> Result<(Contract, Self)> {
      let this = Self::init().await?.add_bootloader().await?;
      let bootloader = deploy_and_init(&this.worker, &BOOTLOADER, &this.root).await?;
      Ok((bootloader, this))
    }

    pub async fn add_bootloader(mut self) -> Result<Self> {
        self.bootloader = Some(deploy_and_init(&self.worker, &BOOTLOADER, &self.root).await?);
        Ok(self)
    }

    pub async fn add_registry(mut self) -> Result<Self> {
        self.registry = Some(deploy_and_init(&self.worker, &REGISTRY, &self.root).await?);
        Ok(self)
    }

    pub fn bootloader(&self) -> Option<&Contract> {
        self.bootloader.as_ref()
    }

    pub fn registry(&self) -> Option<&Contract> {
        self.registry.as_ref()
    }

    pub fn worker(&self) -> &Worker<Sandbox> {
        &self.worker
    }

    pub async fn patch(
        &self,
        bytes: Vec<u8>,
    ) -> Result<ExecutionFinalResult, workspaces::error::Error> {
        self.root
            .call(self.registry().unwrap().id(), "patch")
            .args(bytes)
            .gas(300_000_000_000_000)
            .deposit(parse_near!("10 N"))
            .transact()
            .await
    }
}

// pub fn try_into_bytes(details: CallExecutionDetails) -> anyhow::Result<Vec<u8>> {
//     let outcome = details.outcomes()[1].clone();
//     if let ValueOrReceiptId::Value(result) = outcome.into_result()? {
//         base64::decode(result).map_err(Into::into)
//     } else {
//         anyhow::bail!("Expected value")
//     }
// }

pub async fn init(
    worker: &Worker<impl DevNetwork>,
    root: &Account,
    init_registry: bool,
    simple: bool,
) -> Result<(Contract, Option<(Account, Contract)>)> {
    let bootloader = worker.dev_deploy(&BOOTLOADER).await?;
    let owner_bytes = root.id().as_bytes().to_vec();
    let res = root
        .call(bootloader.id(), "set_owner")
        .args(owner_bytes)
        .gas(300_000_000_000_000)
        .transact()
        .await?;

    assert!(res.is_success(), "set owner");

    let registry = if init_registry {
        let contract = root
            .create_subaccount("contract")
            .initial_balance(parse_near!("20 N"))
            .transact()
            .await?
            .unwrap();
        let registry = contract
            .create_subaccount("registry")
            .initial_balance(parse_near!("10 N"))
            .transact()
            .await?
            .unwrap();
        let registry = registry.deploy(&REGISTRY).await?.unwrap();

        let res = root
            .call(registry.id(), "set_owner")
            .args(root.id().as_bytes().to_vec())
            .gas(300_000_000_000_000)
            // .deposit(parse_near!("1N"))
            .transact()
            .await?;
        assert!(res.is_success(), "Failed to set registry owner");
        // root.batch(&worker, &format!("registry.{}", root.id()).parse()?)
        //     .create_account()
        //     .transfer(parse_near!("10 N"))
        //     .call(Function {
        //         name: "set_owner",
        //         args: root.id().as_bytes().to_vec(),
        //         deposit: 0,
        //         gas: parse_gas!("100 Tgas") as _,
        //     });
        // let registry = worker.dev_deploy(&REGISTRY).await?;

        let res = root
            .call(registry.id(), "patch")
            .args(if simple {
                STATUS_MESSAGE.to_vec()
            } else {
                STATUS_MESSAGE_BINDGEN.to_vec()
            })
            .gas(300_000_000_000_000)
            .deposit(parse_near!("10 N"))
            .transact()
            .await?;
        println!("{:#?}", res);
        assert!(res.is_success(), "failed to uploaded status_message bytes");
        // let boot_bytes = try_into_bytes(res)?;

        // let res = root
        //     .call(worker, registry.id(), "upload")
        //     .args(REGISTRY.to_vec())
        //     .gas(300_000_000_000_000)
        //     .deposit(parse_near!("1N"))
        //     .transact()
        //     .await?;

        // assert!(res.is_success(), "uploaded registry bytes");
        Some((contract, registry))
    } else {
        None
    };

    Ok((bootloader, registry))
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
    println!("Created Contract: {:?}",res.logs());
    Ok(contract)
}

pub async fn init_with_launcher(
    worker: &Worker<impl DevNetwork>,
    root: &Account,
    bootloader: &Account,
) -> Result<(Contract, Contract)> {
    let res = bootloader.deploy(&NEAR_WASM).await?.unwrap();

    let launcher = worker.dev_deploy(&CONTRACT_LAUNCHER).await?;

    let owner_bytes = root.id().as_bytes().to_vec();
    let res = root
        .call(launcher.id(), "set_owner")
        .args(owner_bytes)
        .gas(300_000_000_000_000)
        .transact()
        .await?;

    assert!(res.is_success(), "set owner");
    // let sk = SecretKey::from_seed(KeyType::ED25519, "near_seed");

    // let testnet = worker
    //     .create_tla_and_deploy("testnet".parse()?, sk, &NEAR_WASM)
    //     .await?
    //     .unwrap();
    // let bootloader = testnet
    //     .as_account()
    //     .create_subaccount(worker, "bootloader")
    //     .initial_balance(parse_near!("20 N"))
    //     .transact()
    //     .await?
    //     .unwrap();
    let registry = bootloader
        .create_subaccount("registry")
        .initial_balance(parse_near!("9 N"))
        .transact()
        .await?
        .unwrap();
    let registry = registry.deploy(&REGISTRY).await?.unwrap();

    let res = root
        .call(registry.id(), "set_owner")
        .args(bootloader.id().as_bytes().to_vec())
        .gas(300_000_000_000_000)
        // .deposit(parse_near!("1N"))
        .transact()
        .await?;

    let res = root
        .batch(launcher.id())
        .call(Function::new("update").args_json(json! ({
          "registry": registry.id(),
          "root_account": bootloader.id(),
        })))
        .transact()
        .await?;

    println!("Updated {:#?}", res);
    assert!(res.is_success(), "Failed to set registry owner");
    println!(
        "{}",
        worker
            .view(launcher.id(), "accounts", vec![])
            .await?
            .json::<Value>()?
    );
    // root.batch(&worker, &format!("registry.{}", root.id()).parse()?)
    //     .create_account()
    //     .transfer(parse_near!("10 N"))
    //     .call(Function {
    //         name: "set_owner",
    //         args: root.id().as_bytes().to_vec(),
    //         deposit: 0,
    //         gas: parse_gas!("100 Tgas") as _,
    //     });
    // let registry = worker.dev_deploy(&REGISTRY).await?;

    let res = bootloader
        .call(registry.id(), "patch")
        .args(BOOTLOADER.to_vec())
        .gas(300_000_000_000_000)
        .deposit(parse_near!("10N"))
        .transact()
        .await?;
    println!("{:#?}", res);
    assert!(res.is_success(), "uploaded bootloader bytes");
    // let boot_bytes = try_into_bytes(res)?;

    // let res = root
    //     .call(worker, registry.id(), "upload")
    //     .args(REGISTRY.to_vec())
    //     .gas(300_000_000_000_000)
    //     .deposit(parse_near!("1N"))
    //     .transact()
    //     .await?;

    // assert!(res.is_success(), "uploaded registry bytes");

    Ok((launcher, registry))
}
