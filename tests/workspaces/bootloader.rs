use near_units::{parse_gas, parse_near};
use serde_json::{json, Value};
//use env::is_valid_account_id;

use crate::utils::{init, init_with_launcher};
use near_components::near_sdk::AccountId;


//bootloader initialization test
//Description
//
#[tokio::test]
async fn initialize_correctly() -> anyhow::Result<()> {
    let worker = &workspaces::sandbox().await?;
    let root = worker.root_account();
    let (bootloader, _) = init(worker, &root, false, false).await?;

    let res = bootloader.view(worker, "get_owner", vec![0]).await?;
    let owner = res.json::<String>()?;
    assert_eq!(owner, root.id().to_string());
    Ok(())
}


//CAN DELETE THIS TEST
//Ownership transfer
//Description:
//transfer ownership from original owner to Alice
#[tokio::test]
async fn owner_can_transfer() -> anyhow::Result<()> {
    //initialize worker and root account
    let worker = &workspaces::sandbox().await?;
    let root = worker.root_account();
    //initialize bootloader contract 
    //see ./utils.rs for init(...) implementation
    let (bootloader, _) = init(worker, &root, false, false).await?;

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

//Name: Successive ownership transfer
//Description:
//tests that new owner is able to transfer ownership
//tests that past owners no longer have owner access
//
//Test Steps:
//transfer ownership from root account to Alice, confirm root cannot transfer after ownership transferred to Alice
//transfer ownership from Alice to Kate, confirm Alice cannot transfer after ownership transferred to Kate
#[tokio::test]
async fn successive_ownership_transfer() -> anyhow::Result<()> {
//Configuring Test Env
    //initialize worker and root account
    let worker = &workspaces::sandbox().await?;
    let root = worker.root_account();
    //initialize bootloader contract --> see ./utils.rs for init(...) implementation
    let (bootloader, _) = init(worker, &root, false, false).await?;
    //create alice.root subaccount 
    let alice = root
        .create_subaccount(worker, "alice")
        .transact()
        .await?
        .unwrap();
    //create kate.root subaccount
    let kate = root
    .create_subaccount(worker, "kate")
    .transact()
    .await?
    .unwrap();
    
//Test Part 1: transfer ownership from root to alice
    //root account call "set_owner" to alice 
    let res = root
        .call(worker, bootloader.id(), "set_owner")
        .args(alice.id().as_bytes().to_vec())
        .transact()
        .await?;
    assert!(res.is_success());

    //Confirm owner is Alice
    let res = bootloader.view(worker, "get_owner", vec![0]).await?;
    let owner = res.json::<String>()?;
    assert_eq!(owner, alice.id().to_string());

    //confirm root cannot call "set_owner" again
    let res = root
        .call(worker, bootloader.id(), "set_owner")
        .args(kate.id().as_bytes().to_vec())
        .transact()
        .await;
    assert!(res.is_err());

//Test Part 2: successive ownership transfer 
    //alice account call "set_owner" to kate
    let res = alice
        .call(worker, bootloader.id(), "set_owner")
        .args(kate.id().as_bytes().to_vec())
        .transact()
        .await?;
    assert!(res.is_success());

    //confirm owner is Kate
    let res = bootloader.view(worker, "get_owner", vec![0]).await?;
    let owner = res.json::<String>()?;
    assert_eq!(owner, kate.id().to_string());

    //confirm alice cannot call "set_owner" again
    let res = alice
        .call(worker, bootloader.id(), "set_owner")
        .args(alice.id().as_bytes().to_vec())
        .transact()
        .await;
    assert!(res.is_err());
    Ok(())
}



//CAN DELETE THIS TEST
#[tokio::test]
async fn non_owner_cannot_transfer() -> anyhow::Result<()> {
    let worker = &workspaces::sandbox().await?;
    let root = worker.root_account();
    let (bootloader, _) = init(worker, &root, false, false).await?;

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
    //assert!(res.is_err());
    assert!(res.is_err());
    Ok(())
}



//Name: Non-conformat account id
//Description:
//tests that owner cannot be set to a non-conformat account id
//Test Steps: attempt to set owner a non-conformat account id...
//Reference - near account id rules: https://docs.near.org/docs/concepts/account
#[tokio::test]
async fn cannot_set_owner_to_noncomformantaccount() -> anyhow::Result<()> {
//Configuring Test Env
    //initialize worker and root account
    let worker = &workspaces::sandbox().await?;
    let root = worker.root_account();
    //initialize bootloader contract --> see ./utils.rs for init(...) implementation
    let (bootloader, _) = init(worker, &root, false, false).await?;
    //create alice.root subaccount 
    let alice = root
        .create_subaccount(worker, "alice")
        .transact()
        .await?
        .unwrap();  

    //create invalid account id
    let non_conformant_id = "sub.buy_d1gitz@atata@b0-rg.c_0_m";
    //confirm account id is not valid
    assert!(!near_sdk::env::is_valid_account_id(non_conformant_id.as_bytes()));
//Test: Attempt set owner to non-conformat id
    let res = root
        .call(worker, bootloader.id(), "set_owner")
        .args(non_conformant_id.as_bytes().to_vec())
        .transact()
        .await;
    println!("{:#?}", res);
    assert!(res.is_err());
    Ok(())
}


#[tokio::test]
async fn can_redeploy_simple() -> anyhow::Result<()> {
    deploy_with_simple(true).await
}

#[tokio::test]
async fn can_redeploy_with_bindgen() -> anyhow::Result<()> {
    deploy_with_simple(false).await
}

#[tokio::test]
async fn can_launch() -> anyhow::Result<()> {
    let worker = &workspaces::sandbox().await?;
    let testnet = &workspaces::testnet().await?;
    let testnet = worker
        .import_contract(&"tn".parse().unwrap(), testnet)
        .initial_balance(parse_near!("1000 N"))
        .transact()
        .await?;
    let bootloader = testnet
        .as_account()
        .create_subaccount(worker, "bootloader")
        .initial_balance(parse_near!("200 N"))
        .transact()
        .await?
        .unwrap();
    let root = worker.root_account();
    let (launcher, registry) = init_with_launcher(worker, &root, &bootloader).await?;
    println!("{}", registry.id());

    let res = registry.view(worker, "current_version", vec![]).await?;

    assert_eq!("v0_0_1".to_string(), res.json::<String>()?);

    // let res = root
    //     .call(worker, launcher.id(), "deploy")
    //     .args(format!("v0_0_1.{}", contract.id()).as_bytes().to_vec())
    //     .gas(parse_gas!("250 Tgas") as u64)
    //     .transact()
    //     .await?;
    // println!("{:#?}\nDeployed", res.outcome());
    // assert!(res.is_success());
    let new_account_id = "test.bootloader.tn";
    let new_account = json!({ "account_id": new_account_id });
    let res = root
        .call(worker, launcher.id(), "launch")
        .args_json(new_account.clone())?
        .deposit(parse_near!("10 N"))
        .gas(parse_gas!("300 Tgas") as u64)
        .transact()
        .await?;
    println!("Launched? {:#?}", res);

    let res = worker
        .view_account(&new_account_id.parse().unwrap())
        .await?;
    println!("{:#?}", res);
    let res = worker
        .view(&new_account_id.parse().unwrap(), "get_owner", vec![])
        .await?;
    println!("FINAL RES\n\n\n{:#?}", res.result);
    println!("FINAL RES\n\n\n{:#?}", String::from_utf8(res.result)?);
    Ok(())
}




async fn deploy_with_simple(simple: bool) -> anyhow::Result<()> {
    let worker = &workspaces::sandbox().await?;
    let root = worker.root_account();
    let (bootloader, registry) = init(worker, &root, true, simple).await?;
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
    let args = if simple {
        hello.clone()
    } else {
        json!({ "message": hello })
    };
    let res = root
        .call(worker, bootloader.id(), "update_message")
        .args_json(args)?
        .transact()
        .await?;

    let res = bootloader
        .view(worker, "get_message", vec![])
        .await?
        .json::<Value>()?;
    println!("{:#?}", res);
    assert_eq!(res, hello);
    Ok(())
}
