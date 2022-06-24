use near_units::parse_gas;
use serde_json::{json, Value};

use crate::utils::init;

#[tokio::test]
async fn initialize_correctly() -> anyhow::Result<()> {
    let worker = &workspaces::sandbox().await?;
    let root = worker.root_account();
    let (bootloader, _) = init(worker, &root, false).await?;

    let res = bootloader.view(worker, "get_owner", vec![0]).await?;
    let owner = res.json::<String>()?;
    assert_eq!(owner, root.id().to_string());
    Ok(())
}

#[tokio::test]
async fn owner_can_transfer() -> anyhow::Result<()> {
    let worker = &workspaces::sandbox().await?;
    let root = worker.root_account();
    let (bootloader, _) = init(worker, &root, false).await?;

    let alice = root
        .create_subaccount(worker, "alice")
        .transact()
        .await?
        .unwrap();
    let res = root
        .call(worker, bootloader.id(), "set_owner")
        .args(alice.id().as_bytes().to_vec())
        .transact()
        .await?;
    assert!(res.is_success());
    let res = bootloader.view(worker, "get_owner", vec![0]).await?;
    let owner = res.json::<String>()?;
    assert_eq!(owner, alice.id().to_string());
    Ok(())
}

#[tokio::test]
async fn non_owner_cannot_transfer() -> anyhow::Result<()> {
    let worker = &workspaces::sandbox().await?;
    let root = worker.root_account();
    let (bootloader, _) = init(worker, &root, false).await?;

    let alice = root
        .create_subaccount(worker, "alice")
        .transact()
        .await?
        .unwrap();
    let res = alice
        .call(worker, bootloader.id(), "set_owner")
        .args(alice.id().as_bytes().to_vec())
        .transact()
        .await;
    println!("{:#?}", res);
    assert!(res.is_err());
    Ok(())
}

#[tokio::test]
async fn can_redeploy() -> anyhow::Result<()> {
    let worker = &workspaces::sandbox().await?;
    let root = worker.root_account();
    let (bootloader, registry) = init(worker, &root, true).await?;
    let (contract, registry) = registry.unwrap();
    println!("{}", registry.id());

    let res = registry.view(worker, "current_version", vec![]).await?;

    assert_eq!("v0_0_1".to_string(), res.json::<String>()?);

    let res = root
        .call(worker, bootloader.id(), "deploy")
        .args(format!("v0_0_1.{}", contract.id()).as_bytes().to_vec())
        .gas(parse_gas!("250 Tgas") as u64)
        .transact()
        .await?;
    println!("{:#?}\nDeployed", res.outcome());
    assert!(res.is_success());
    let hello = json!({ "text": "hello world" });
    let res = root
        .call(worker, bootloader.id(), "update_message")
        .args_json(hello.clone())?
        .transact()
        .await?;

    let res = bootloader
        .view(worker, "get_message", vec![])
        .await?.json::<Value>()?;
    println!("{:#?}", res);
    assert_eq!(res, hello);
    Ok(())
}
