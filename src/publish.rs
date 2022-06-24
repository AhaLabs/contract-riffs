use crate::{near_sdk::env, owner::Owner, reg};

use near_sdk::Gas;
use near_units::parse_gas;

const ONE_TGAS: Gas = Gas(parse_gas!("1 Tgas") as u64);

fn call_registry(function_name: &str) {
    env::promise_return(reg::promise_create_args_from_input(
        &format!("registry.{}", env::current_account_id()),
        function_name,
        env::attached_deposit(),
        // Subtract 1 Tgas just as a buffer
        (env::prepaid_gas() - (env::used_gas() + ONE_TGAS)).0,
    ));
}

#[no_mangle]
pub fn publish_patch() {
    Owner::assert();
    call_registry("patch")
}

#[no_mangle]
pub fn publish_minor() {
    Owner::assert();
    call_registry("minor")
}

#[no_mangle]
pub fn publish_major() {
    Owner::assert();
    call_registry("major")
}
