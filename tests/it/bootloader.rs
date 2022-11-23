#![allow(unused_must_use)]
use near_units::parse_near as near;
use serde_json::json;
use workspaces::Contract;

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
    let res = factory
        .view("current_version", vec![])
        .await?
        .json::<String>()?;
    assert_eq!("v0_0_1".to_string(), res);
    Ok(())
}

#[tokio::test]
async fn can_create_account_from_factory() -> anyhow::Result<()> {
    let testenv = &TestEnv::init().await?;
    let factory = &testenv.factory("factory", Contracts::Bootloader).await?;
    let alice = &testenv.create_subaccount_and_deploy(factory, ALICE).await?;
    let bootloader = &testenv.bootloader().await?;
    assert_equal_contracts(alice, bootloader).await;

    Ok(())
}

#[tokio::test]
async fn can_create_account_from_factory_and_be_locked() -> anyhow::Result<()> {
    let testenv = &TestEnv::init().await?;
    let factory = &testenv
        .factory("factory", Contracts::BootloaderLocked)
        .await?;
    let alice = &testenv.create_subaccount_and_deploy(factory, ALICE).await?;
    let factory_two = &testenv.factory("factory_2", Contracts::Factory).await?;
    testenv
        .redeploy(&testenv.root, alice, factory_two)
        .await?
        .assert_failure();
    testenv
        .patch(factory.id(), Contracts::Factory.into())
        .await?
        .assert_success();
    testenv
        .redeploy(&testenv.root, alice, factory)
        .await?
        .assert_success();
    assert_equal_contracts(alice, factory_two).await;
    Ok(())
}

#[tokio::test]
async fn can_upgrade_from_factory() -> anyhow::Result<()> {
    let testenv = &TestEnv::init().await?;
    let root = &testenv.root;
    let factory = &testenv.factory("factory", Contracts::Bootloader).await?;
    let alice = &testenv.create_subaccount_and_deploy(factory, ALICE).await?;
    testenv
        .patch(factory.id(), Contracts::Factory.into())
        .await?
        .assert_success();

    testenv
        .redeploy(root, alice, factory)
        .await?
        .assert_success();

    assert_equal_contracts(alice, factory);
    Ok(())
}

#[tokio::test]
async fn can_create_factory_of_factories() -> anyhow::Result<()> {
    let testenv = &TestEnv::init().await?;
    let factory = &testenv.factory("factory", Contracts::Factory).await?;
    let new_factory = &testenv.create_subaccount_and_deploy(factory, ALICE).await?;
    assert_equal_contracts(factory, new_factory).await;
    Ok(())
}

#[tokio::test]
async fn can_launch_with_launcher() -> anyhow::Result<()> {
    let (launcher, testenv) = TestEnv::with_launcher().await?;
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

async fn assert_equal_contracts(a: &Contract, b: &Contract) {
    assert_eq!(
        a.view_account().await.unwrap().code_hash,
        b.view_account().await.unwrap().code_hash,
        "Contract {}'s bytes differ from {}",
        a.id(),
        b.id()
    )
}
