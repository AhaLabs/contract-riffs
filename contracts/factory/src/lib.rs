//! # Bootloader Contract
//!
//! This contract has the the minimum code needed to claim ownership and be able to re-deploy the contract as the owner.
//!
//! The following brings all needed types into scope and exports the necessary contract methods
//! ```
//! pub use near_riffs::prelude::*;
//! ```
//!
//! This is equivalent to:
//! ```
//!  // Is ownable, e.i. stores owner in storage at "OWNER"
//! pub use near_riffs::owner::*;
//!  // Uses ownable to check owner before deploying contract
//! pub use near_riffs::deploy::*;
//!
//! // If a type implements IntoKey and BorshSerialize + BorshDeserialize
//! // then it can become a lazy riff. That is have state separate from the contract's main state.
//! pub use super::lazy::Lazy;
//! pub use super::IntoKey;
//! ```

pub use near_riffs_core::*;
pub use near_riffs_launcher::*;

/// This mod provides the WIT types that describe the Application Contract Interface (ACI)
#[allow(dead_code, unused_variables)]
mod private {
    use near_riffs::{
        near_sdk::{AccountId, PublicKey},
        version::Version,
        witgen,
    };
    use near_riffs_launcher::Launcher;

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
    /// Create Subaccount and deploy contract. The signer is the owner of the new contract.
    /// Currently the most recent contract in the registry is used
    /// @change
    pub fn create_subaccount_and_deploy(new_account_id: AccountId, new_public_key: PublicKey) {}

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
}
