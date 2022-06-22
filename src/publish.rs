use crate::{deploy::promise::promise_create_proxy, owner::Owner, near_sdk::env};

use near_sdk::Gas;
use near_units::parse_gas;

const ONE_TGAS: Gas = Gas(parse_gas!("1 Tgas") as u64);

#[no_mangle]
pub fn publish_patch() {
    Owner::assert();
    promise_create_proxy(
        &format!("registry.{}", env::current_account_id()),
        "patch",
        env::attached_deposit(),
        // Subtract 1 Tgas just as a buffer
        env::prepaid_gas() - (env::used_gas() + ONE_TGAS),
    );
}
