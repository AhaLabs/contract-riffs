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
