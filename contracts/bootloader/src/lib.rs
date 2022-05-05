//! # Bootloader Contract
//!
//! The core idea for this contract is to be the minimum code needed to claim ownership
//! of a contract and be able to re-deploy the contract as the owner.
//!

/// Is ownable, e.i. stores owner in storage at "OWNER"
pub use contract_utils::owner::*;
/// Uses ownable to check owner before deploying contract
pub use contract_utils::upgrade::*;
