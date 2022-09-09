#![allow(unused_must_use)]
use near_units::parse_near;
use serde_json::json;

use crate::utils::{AssertResult, IntoVec, TestEnv};

#[tokio::test]
async fn initialize_correctly() -> anyhow::Result<()> {
    let (bootloader, test_env) = TestEnv::with_bootloader().await?;
    let res = bootloader.view("get_owner", vec![]).await?;
    let owner = String::from_utf8(res.result)?;
    assert_eq!(owner, test_env.root.id().to_string());
    Ok(())
}

#[tokio::test]
async fn owner_can_transfer() -> anyhow::Result<()> {
    let (bootloader, test_env) = TestEnv::with_bootloader().await?;

    test_env
        .root
        .call(bootloader.id(), "set_owner")
        .args(test_env.alice.to_vec())
        .transact()
        .await?
        .assert_success();
    let owner = bootloader
        .view("get_owner_json", vec![])
        .await?
        .json::<String>()?;

    assert_eq!(owner, test_env.alice.id().to_string());
    Ok(())
}

#[tokio::test]
async fn non_owner_cannot_transfer() -> anyhow::Result<()> {
    let (bootloader, test_env) = TestEnv::with_bootloader().await?;

    test_env
        .alice
        .call(bootloader.id(), "set_owner")
        .args(test_env.alice.to_vec())
        .transact()
        .await?
        .assert_failure();
    Ok(())
}

#[tokio::test]
async fn can_create_registry() -> anyhow::Result<()> {
    let testenv = TestEnv::init().await?;
    let registry = testenv.registry_with_bootlader().await?;
    let res = registry.view("current_version", vec![]).await?;

    assert_eq!("v0_0_1".to_string(), res.json::<String>()?);

    Ok(())
}

#[tokio::test]
async fn can_launch_with_launcher() -> anyhow::Result<()> {
    let (launcher, testenv) = TestEnv::with_lancher().await?;
    let registry = testenv.registry_with_bootlader().await?;
    let root_contract = testenv.deploy_root_contract().await?;
    let account_id = &format!("charlie.{}", root_contract.id()).parse().unwrap();
    let args = json!({
    "account_id": account_id,
    "registry": Some(registry.id()),
    "root_account": Some(root_contract.id())
    });
    let res = launcher
        .call("launch")
        .args_json(args)
        .max_gas()
        .deposit(parse_near!("6 N"))
        .transact()
        .await?
        .assert_success();
    println!("{:#?}", res);
    println!(
        "{}",
        testenv
            .worker
            .view(account_id, "get_owner_json", vec![],)
            .await?
            .json::<String>()?
    );
    Ok(())
}
