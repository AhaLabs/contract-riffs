# Bootme: contract standards for publishing, deploying, and upgrading contracts

Currently NEAR smart contracts written in Rust are singletons; there is one struct or enum that represents the state of the contract. This state is stored in contract storage with the aptly named key `STATE`. Each time a contract method is called, this state is read, potentially updated, and written back to storage.

## Contract Component

The concept of this singleton stored with a unique key in storage is called a _contract component_. Thus the current `STATE` singleton used by `near_sdk_rs` is a contract component.

This project allows adding multiple components per contract.

A component must have the following:

- Be Borsh serializable; to be written and read to storage
- Have a unique key in storage

That's it!

### Example: Owner Component

This is the core component of a contract. It stores the `AccountId` with the unique key `OWNER`. It has two methods `set_owner` and `get_owner`.  The former can be called initially by any account once and then only by the owner.

Any contract that includes an owner component will then be _ownable_ and can restrict certain methods to the owner's account. For example having the following in your contract's `lib.rs` file:

```rust
pub use contract_utils::owner::*;
```

Note `pub` here.  This exports the owner's components methods.

### Example: Deploy Component

The `Owner` component is a _stateful_ component, however, the `Deploy` component is a functional component which depends on the `Owner` component. It provides a `deploy` method that only the owner can call. This method requires the address of a contract which has a registry, which returns the bytes of a published contract.

```rust
pub use contract_utils::deploy::*;
```

## Bootloader

The bootloader contract is made up of these two core components. It's named after an [Operating System bootloader](https://en.wikipedia.org/wiki/Bootloader) which contains minimal the code to load the rest of OS.

Once deployed the owner account can call `deploy` to redeploy the contract into any contract that has at least the core components.

To include both these core components you simple include the prelude:

```rust
pub use contract_utils::prelude::*;
```

## Registry

This is a special contract that lives at a sub-account, registry, `registry.contract.near`. This registry allows you to publish versions of a contract. It four methods `patch`, `minor`, `major`, and `fetch`.  The first three are for publishing and increase the version of the contract accordingly, with the bytes of the contract attached. `fetch` optionally takes a version, e.g. `"0_0_1"`, otherwise assumes of the latest version, and returns the stored bytes.

## Launcher

This contract allows an account to be claimed and a contract deployed in one step, setting the owner of the new contract. By default this contract will be the bootloader contract.

This new contract can then be upgraded into any contract that supports the core components, ensuring that the contract is always redeployable.

## Benefits of components

Currently when upgrading a contract with new state the State, component must migrate, which is a non-trivial step. However, adding a new component with a unique key does not require a migration.

### Lazy loading

A common pattern in contracts is to use a lazy option in the `STATE` struct, which essentially acts like a component and is only read in when required.

Stateful components meet the requirements for a new [`Lazy` trait](./src/lazy/mod.rs), allowing them to loaded using a lazy option and be referenced by other components. This allows the only the components needed for the current execution to be loaded in, saving on gas.


## Using library

This library depends on `near-sdk-rs` and re-exports it if you want to use the same version. However, if you use this version you will need to add the `wee_alloc` feature:

```toml
contract-utils = { version = "0.0.1", features = ["wee_alloc"]}

```
