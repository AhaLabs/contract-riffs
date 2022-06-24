use near_units::{parse_gas, parse_near};
use workspaces::operations::Function;
use workspaces::prelude::*;
use workspaces::result::{CallExecutionDetails, ValueOrReceiptId};
use workspaces::types::{KeyType, SecretKey};
use workspaces::{Account, Contract, DevNetwork, Worker};

lazy_static_include::lazy_static_include_bytes! {
  pub BOOTLOADER => "./target/res/bootloader.wasm",
  pub REGISTRY => "./target/res/contract_registry.wasm",
  pub STATUS_MESSAGE => "./target/res/status_message.wasm",
  pub CONTRACT_LAUNCHER => "./target/res/contract_launcher.wasm",
  pub NEAR_WASM => "./target/res/near.wasm"

}

pub fn try_into_bytes(details: CallExecutionDetails) -> anyhow::Result<Vec<u8>> {
    let outcome = details.outcomes()[1].clone();
    if let ValueOrReceiptId::Value(result) = outcome.into_result()? {
        base64::decode(result).map_err(Into::into)
    } else {
        anyhow::bail!("Expected value")
    }
}

pub async fn init(
    worker: &Worker<impl DevNetwork>,
    root: &Account,
    init_registry: bool,
) -> anyhow::Result<(Contract, Option<(Account, Contract)>)> {
    let bootloader = worker.dev_deploy(&BOOTLOADER).await?;
    let owner_bytes = root.id().as_bytes().to_vec();
    let res = root
        .call(worker, bootloader.id(), "set_owner")
        .args(owner_bytes)
        .gas(300_000_000_000_000)
        .transact()
        .await?;

    assert!(res.is_success(), "set owner");

    let registry = if init_registry {
        let contract = root
            .create_subaccount(worker, "contract")
            .initial_balance(parse_near!("20 N"))
            .transact()
            .await?
            .unwrap();
        let registry = contract
            .create_subaccount(worker, "registry")
            .initial_balance(parse_near!("10 N"))
            .transact()
            .await?
            .unwrap();
        let registry = registry.deploy(worker, &REGISTRY).await?.unwrap();

        let res = root
            .call(worker, registry.id(), "set_owner")
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
            .call(worker, registry.id(), "patch")
            .args(STATUS_MESSAGE.to_vec())
            .gas(300_000_000_000_000)
            .deposit(parse_near!("1N"))
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

pub async fn init_with_launcher(
    worker: &Worker<impl DevNetwork>,
    root: &Account,
    init_registry: bool,
) -> anyhow::Result<(Contract, Option<(Account, Contract)>)> {
    let launcher = worker.dev_deploy(&CONTRACT_LAUNCHER).await?;
    let owner_bytes = root.id().as_bytes().to_vec();
    let res = root
        .call(worker, launcher.id(), "set_owner")
        .args(owner_bytes)
        .gas(300_000_000_000_000)
        .transact()
        .await?;

    assert!(res.is_success(), "set owner");

    let registry = if init_registry {
        let sk = SecretKey::from_seed(KeyType::ED25519, "near_seed");

        // let testnet = worker
        //     .create_tla_and_deploy("testnet".parse()?, sk, &NEAR_WASM)
        //     .await?
        //     .unwrap();
        let contrct = Contract::new()
        let bootloader = testnet
            .as_account()
            .create_subaccount(worker, "bootloader")
            .initial_balance(parse_near!("20 N"))
            .transact()
            .await?
            .unwrap();
        let registry = bootloader
            .create_subaccount(worker, "registry")
            .initial_balance(parse_near!("10 N"))
            .transact()
            .await?
            .unwrap();
        let registry = registry.deploy(worker, &REGISTRY).await?.unwrap();

        let res = root
            .call(worker, registry.id(), "set_owner")
            .args(bootloader.id().as_bytes().to_vec())
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

        let res = bootloader
            .call(worker, registry.id(), "patch")
            .args(BOOTLOADER.to_vec())
            .gas(300_000_000_000_000)
            .deposit(parse_near!("1N"))
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
        Some((bootloader, registry))
    } else {
        None
    };

    Ok((launcher, registry))
}

pub async fn patch_contract(
  worker: &Worker<impl DevNetwork>){
    worker.
  }