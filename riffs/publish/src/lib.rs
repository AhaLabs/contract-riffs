use near_riffs::{
    near_sdk::{env, Gas},
    near_units::parse_gas,
    reg,
};

use near_riffs_core::Owner;

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
    Owner::assert_owner();
    call_registry("patch")
}

#[no_mangle]
pub fn publish_minor() {
    Owner::assert_owner();
    call_registry("minor")
}

#[no_mangle]
pub fn publish_major() {
    Owner::assert_owner();
    call_registry("major")
}
