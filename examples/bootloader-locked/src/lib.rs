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
