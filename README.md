# Near Riffs: contract standards for publishing, deploying, and upgrading contracts

Currently NEAR smart contracts written in Rust using `near_sdk_rs` are singletons; there is one struct or enum that represents the state of the contract. This state is stored in contract storage with the aptly named key `STATE`. Each time a contract method is called, this state is read and potentially updated in storage.

## Contract Riff

The concept of this singleton stored with a unique key in storage is called a _contract riff_. Thus the current `STATE` singleton used by `near_sdk_rs` is a contract riff.

A riff must have the following:

- Be Borsh serializable; to be written and read to storage
- Have a unique key in storage

That's it! Since their keys are unique a contract can have several riffs.

### Example: Owner Riff

This is the core riff of a contract. It stores the `AccountId` with the unique key `OWNER`. It has two methods `set_owner` and `get_owner`.  The former can be called initially by any account once and then only by the owner.

Any contract that includes an owner riff will then be _ownable_ and can restrict certain methods to the owner's account. For example having the following in your contract's `lib.rs` file:

```rust
pub use near_riffs_core::owner::*;
```

Note `pub` here.  This exports the owner's riffs methods.

### Example: Deploy Riff

The `Owner` riff is a _stateful_ riff, however, the `Deploy` riff is functional and depends on the `Owner` riff. It provides a `deploy` method that only the owner can call. This method requires the address of a registry contract, which returns the bytes of a published contract.

```rust
pub use near_riffs_core::deploy::*;
```

## Bootloader

The bootloader contract is made up of these two core riffs. It's named after an [Operating System bootloader](https://en.wikipedia.org/wiki/Bootloader) which contains minimal the code to load the rest of OS.

Once deployed the owner account can call `deploy` to redeploy the contract into any contract that has at least the core riffs.

To include both these core riffs you simple include the prelude:

```rust
pub use near_riffs_core::*;
```

## Registry Riff

A registry riff allows you to publish versions of a contract. Its four methods are `patch`, `minor`, `major`, and `fetch`.  The first three are for publishing and increase the version of the contract accordingly, with the bytes of the contract attached. `fetch` optionally takes a version, e.g. `"0_0_1"`, otherwise assumes of the latest version, and returns the bytes.

## Factory Riff

A factory riff extends the registry riff and provides a `create_subaccount_and_deploy` method, which unsurprisingly creates a new subaccount, deploys the contract found in the contract's registry, and initializes it by setting the owner.  Then the deployed contract can be further initialized by the owner.

An intsance of this contract is deployed to [`factory-riff.testnet`](https://raen.dev/admin/#/factory-riff.testnet). This factory's registry contains the factory contract itself! So to upgrade the contract, you call `patch`/`minor`/`majory` with the contract bytes, then call `deploy` passing the contract's name.

## Launcher Contract

This contract allows an account to be claimed and a contract deployed in one step, setting the owner of the new contract. By default this contract will be the bootloader contract.

This new contract can then be upgraded into any contract that supports the core riffs, ensuring that the contract is always redeployable.

A launcher requires a `root_account` contract which provides a `create_account_and_deploy`; this would extend the current API of the root contracts found at `near` on mainnet and `testnet` on testnet. See [linkdrop](./contracts/likndrop) for an example.

## Benefits of riffs

Currently when upgrading a contract with new state the riff located at `STATE` must migrate, which is a non-trivial step. However, adding a new riff with a unique key does not require a migration since they won't overlap.

### Lazy loading

A common pattern in contracts is to use a lazy option in the `STATE` struct, which essentially acts like a riff and is only read in when required.

Stateful riffs meet the requirements for a new [`Lazy` trait](./src/lazy/mod.rs), allowing them to loaded using a lazy option and be referenced by other riffs. This saves gas by only loading in the riffs needed for the current execution. 

### `reg` module

Another feature of this library is the `reg` module, which provides a way to use registers (buckets of data outside the contract) when calling 

## Using library

This library depends on `near-sdk-rs` and re-exports it if you want to use the same version. However, if you use this version you will need to add the `wee_alloc` feature:

```toml
near-riffs = { version = "0.0.1", features = ["wee_alloc"]}

```
