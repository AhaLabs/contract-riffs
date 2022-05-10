use near_units::parse_near;
use workspaces::prelude::DevAccountDeployer;
use workspaces::{Account, Contract, DevNetwork, Worker};

lazy_static_include::lazy_static_include_bytes! {
  pub BOOTLOADER => "./target/wasm32-unknown-unknown/release/bootloader.wasm",
  pub REGISTRY => "./target/wasm32-unknown-unknown/release/contract_registry.wasm",
}

pub async fn init(
    worker: &Worker<impl DevNetwork>,
    root: &Account,
    registry: bool,
) -> anyhow::Result<(Contract, Option<Contract>)> {
    let bootloader = worker.dev_deploy(&BOOTLOADER).await?;
    let owner_bytes = root.id().as_bytes().to_vec();
    println!("{}", owner_bytes.len());
    let res = root
        .call(worker, bootloader.id(), "set_owner")
        .args(owner_bytes)
        .gas(300_000_000_000_000)
        .transact()
        .await?;

    assert!(res.is_success(), "set owner");

    let registry = if registry {
        let registry = worker.dev_deploy(&REGISTRY).await?;

        let res = root
            .call(&worker, registry.id(), "upload")
            .args(BOOTLOADER.to_vec())
            .gas(300_000_000_000_000)
            .deposit(parse_near!("1N"))
            .transact()
            .await?;

        assert!(res.is_success(), "uploaded bootloader bytes");

        let res = root
            .call(&worker, registry.id(), "upload")
            .args(REGISTRY.to_vec())
            .gas(300_000_000_000_000)
            .deposit(parse_near!("1N"))
            .transact()
            .await?;

        assert!(res.is_success(), "uploaded registry bytes");
        Some(registry)
    } else {
        None
    };

    Ok((bootloader, registry))
}
