//! # Near Components
//!
//! Composible riffs for NEAR smart contracts

pub use near_sdk;
pub use near_units;
pub use witgen::witgen;

pub mod account;
pub mod input;
pub mod lazy;
pub mod promise;
pub mod reg;
pub mod storage;
pub mod version;


pub mod prelude {
    pub use super::lazy::Lazy;
    pub use super::IntoKey;
}


pub trait IntoKey {
    fn into_storage_key() -> Vec<u8>;
}


