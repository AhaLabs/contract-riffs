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
        .call(worker, &bootloader.id(), "set_owner")
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
        .call(worker, &bootloader.id(), "set_owner")
        .args(alice.id().as_bytes().to_vec())
        .transact()
        .await?;
    assert!(res.is_failure());
    Ok(())
}