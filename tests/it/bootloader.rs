#![allow(unused_must_use)]
use near_units::parse_near as near;
use serde_json::json;
use workspaces::Account;

const SIX_NEAR: u128 = near!("6 N");

use crate::utils::{AccountIdTools, AssertResult, Contracts, IntoVec, TestEnv, ALICE};

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
        .args(ALICE.to_vec())
        .transact()
        .await?
        .assert_success();
    let owner = bootloader
        .view("get_owner_json", vec![])
        .await?
        .json::<String>()?;

    assert_eq!(owner, ALICE.to_string());
    Ok(())
}

#[tokio::test]
async fn non_owner_cannot_transfer() -> anyhow::Result<()> {
    let (bootloader, test_env) = TestEnv::with_bootloader().await?;
    let alice = test_env.alice().await?;
    alice
        .call(bootloader.id(), "set_owner")
        .args(alice.to_vec())
        .transact()
        .await?
        .assert_failure();
    Ok(())
}

#[tokio::test]
async fn can_create_registry() -> anyhow::Result<()> {
    let testenv = TestEnv::init().await?;
    let registry = testenv.registry(Contracts::Bootloader).await?;
    let res = registry.view("current_version", vec![]).await?;

    assert_eq!("v0_0_1".to_string(), res.json::<String>()?);

    Ok(())
}

#[tokio::test]
async fn can_create_factory() -> anyhow::Result<()> {
    let testenv = TestEnv::init().await?;
    let factory_id = testenv.root.id().subaccount("factory");
    let factory = testenv.factory("factory", Contracts::Bootloader).await?;
    assert_eq!(&factory_id, factory.id());
    let res = factory.view("current_version", vec![]).await?;
    assert_eq!("v0_0_1".to_string(), res.json::<String>()?);
    Ok(())
}

#[tokio::test]
async fn can_create_account_from_factory() -> anyhow::Result<()> {
    let testenv = TestEnv::init().await?;
    let factory = testenv.factory("factory", Contracts::Bootloader).await?;
    let alice = "alice";
    let new_account_id = factory.id().subaccount(alice);
    let secret_key = new_account_id.to_sk();
    testenv
        .root
        .call(factory.id(), "create_subaccount_and_deploy")
        .args_json(json!({
            "new_account_id": new_account_id,
            "new_public_key": secret_key.public_key(),
        }))
        .deposit(SIX_NEAR)
        .max_gas()
        .transact()
        .await?
        .assert_success();

    let alice = Account::from_secret_key(new_account_id, secret_key, &testenv.worker);
    println!("{:?}", alice.view_account().await?);
    Ok(())
}

#[tokio::test]
async fn can_launch_with_launcher() -> anyhow::Result<()> {
    let (launcher, testenv) = TestEnv::with_lancher().await?;
    let registry = testenv.registry(Contracts::Bootloader).await?;
    let root_contract = testenv.deploy_root_contract().await?;
    let account_id = &format!("charlie.{}", root_contract.id()).parse().unwrap();
    let args = json!({
    "account_id": account_id,
    "registry": Some(registry.id()),
    "root_account": Some(root_contract.id())
    });
    launcher
        .call("launch")
        .args_json(args)
        .max_gas()
        .deposit(near!("6 N"))
        .transact()
        .await?
        .assert_success();
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
