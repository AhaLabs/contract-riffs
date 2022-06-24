# Bootme: contract standards for publishing, deploying, and upgrading contracts

Currently NEAR smart contracts written in Rust are singletons; that is one struct or enum represents the state of the contract. This state is stored in with the aptly named string `STATE`. Each time a contract method is called, this state is read potentially updated and written back to storage.

## Contract Component

The concept of this singleton stored in a unique place in storage is called a _contract component_. This project extends the currently `STATE` component to allow for multiple components per contract.

A component must have the following:

- Be Borsh serializable; to be written and read to storage
- Have a unique key in storage

That's it!

### Example: Owner Component

This is the core component of a contract. It stores the AccountId with the unique key `OWNER`. It has two methods `set_owner` and `get_owner`.  The former can be called by any account once and then only by the owner.

Any contract that includes a owner will then be _ownable_ and restrict certain methods to the owner. For example having the following in your contract's `lib.rs` file:

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

The bootloader contract is thus made up of these two components. It's named after an Operating System bootloader which contains minimal the code to load the rest of OS.

Once deployed the owner can call `deploy` to redeploy the contract into any contract that has at least the two core components.

To include both these core components you simple include the prelude:

```rust
pub use contract_utils::prelude::*;
```

## Registry

This is a special contract that lives at a sub-account, registry, `registry.contract.near`. This registry allows you to publish versions of a contract. It four methods `patch`, `minor`, `major`, and `fetch`.  The first three are for publishing and increase the version of the contract accordingly, with the bytes of the contract attached. `fetch` optionally takes a version, otherwise returns the bytes of the latest version.

## Launcher

This contract allows an account to be claimed and a contract deployed in one step, setting the owner of the new contract. By default this contract will be the bootloader contract.

This new contract can then be upgraded into any contract that supports the core components, ensuring that the contract is always redeployable.

## Benefits of components

Currently when upgrading a contract with new state the State, component must migrate, which is a non-trivial step. However, adding a new component with new state doesn't require any migration, or if it does it can be done lazily.

### Lazy loading

A common pattern in contracts is to use a lazy option in your main `STATE` struct to store data. This essentially acts like a component and is only read in when required. 

Stateful components meet the requirements for a new `Lazy` trait, allowing them to be referenced by other components and loaded when needed. This allows the only the parts of storage needed for the current execution to loaded in, saving on gas.
