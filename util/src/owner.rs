use std::{fmt, io};

use crate::{
    account::{self, FixedAccountId},
    lazy::*,
    reg, IntoKey,
};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, require,
};

pub const OWNER_KEY: &str = "OWNER";

#[derive(Debug, BorshSerialize)]
pub struct Owner(FixedAccountId);

impl IntoKey for Owner {
    fn into_storage_key() -> Vec<u8> {
        OWNER_KEY.as_bytes().to_vec()
    }
}

impl BorshDeserialize for Owner {
    fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        <String as BorshDeserialize>::deserialize(buf).and_then(|s| {
            Self::try_from(s).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ParseFixedAccountIdError {}

impl std::error::Error for ParseFixedAccountIdError {}

impl fmt::Display for ParseFixedAccountIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "the Owner Account ID is invalid")
    }
}

impl TryFrom<String> for Owner {
    type Error = ParseFixedAccountIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let bytes = value.as_bytes().to_vec().into_boxed_slice();
        if bytes.len() <= 64 {
            let mut res = [0u8; 64];
            res.copy_from_slice(&bytes[0..64]);
            Ok(Self(res))
        } else {
            Err(Self::Error {})
        }
    }
}

pub trait Ownable {
    fn assert_owner(&self) {
        Owner::assert()
    }
}

impl<Item> Ownable for Item {}

impl Owner {
    pub fn is_set() -> bool {
        env::storage_has_key(OWNER_KEY.as_bytes())
    }

    pub fn assert() {
        require!(Owner::is_set(), "No owner set");
        require!(Owner::is_predecessor(), "Predecessor is not owner");
    }
    pub fn is_predecessor() -> bool {
        let predecessor_account_id = account::predecessor_account_id();
        Owner::get().unwrap() == predecessor_account_id
    }

    fn set_from_input() -> bool {
        if Owner::is_set() {
            Owner::assert()
        }
        let mut container = Owner::get_lazy();
        container.set_reg(reg::input(0))
    }

    pub fn get() -> Option<FixedAccountId> {
        Owner::is_set().then(|| {
            reg::storage_read(OWNER_KEY.as_bytes(), 0);
            account::read_register(0)
        })
    }

    pub fn get_str() -> String {
        Owner::is_set()
            .then(|| unsafe {
                String::from_utf8_unchecked(env::storage_read(&OWNER_KEY.as_bytes()).unwrap())
            })
            .unwrap_or_else(|| env::panic_str("Owner not set"))
    }
}

#[no_mangle]
pub fn set_owner() {
    Owner::set_from_input();
}

#[no_mangle]
pub fn get_owner() {
    let s = &format!("\"{}\"", Owner::get_str());
    env::value_return(s.as_bytes())
}
