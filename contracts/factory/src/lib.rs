//! # Factory Contract
//!
//! This is an instance of the factory riff, which allows creating subaccounts and deploying the contract in the contract's registry.
//!
//! ```

pub use near_riffs_factory::*;

/// This mod provides the WIT types that describe the Application Contract Interface (ACI)
#[allow(dead_code, unused_variables)]
mod private {
    use near_riffs::{
        near_sdk::{AccountId, PublicKey},
        witgen,
    };

    #[witgen]
    /// @change
    pub fn set_owner(account_id: AccountId) {}

    #[witgen]
    pub fn get_owner_json() -> AccountId {
        todo!("")
    }

    #[witgen]
    pub fn get_owner() -> Vec<u8> {
        todo!("")
    }

    #[witgen]
    /// Redeploys contract from  provided registry.
    /// e.g. `v0_0_1.contract.testnet`
    /// @change
    pub fn deploy(account_id: AccountId) {}

    #[witgen]
    /// Create new account without linkdrop and deposit passed funds (used for creating sub accounts directly).
    /// Then Deploy a contract and optionally call an init method
    /// If a public key is not provided, it will use the key of the signer
    /// If an owner_id is not provided, it will use the predecessor_account_id
    /// Requires at least 6N = 6000000000000000000000000
    /// @change
    pub fn create_subaccount_and_deploy(
        new_account_id: AccountId,
        new_public_key: Option<PublicKey>,
        owner_id: Option<AccountId>,
    ) {
    }

    #[witgen]
    /// get current versions in registry
    pub fn versions() -> Vec<String> {
        todo!()
    }

    #[witgen]
    /// get current version in registry
    pub fn current_version() -> String {
        todo!()
    }

    #[witgen]
    /// A file with an account id on each line.
    /// @format data-url
    pub type DataUrl = String;

    /// Publish a contract to the registry
    /// @change
    #[witgen]
    pub fn patch_contract(bytes: DataUrl) {}

    /// Get bytes of current contract in registry
    #[witgen]
    pub fn fetch() -> Vec<u8> {
        todo!()
    }
}
