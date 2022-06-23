use near_sdk::{env, sys};

pub const EVICTED_REGISTER: u64 = std::u64::MAX - 1;
pub const DATA_REGISTER: u64 = std::u64::MAX - 2;

pub enum Registers {
    Input = 1,
    CurrentAccountId = 2,
    PredecessorAccountId = 3,
    SignerAccountId = 4,
    SignerAccountPK = 5,
    StorageRead = 6,
    StorageWriteEviction = 7,
}

impl From<Registers> for u64 {
    fn from(reg: Registers) -> Self {
        reg as u64
    }
}

pub fn length(reg_id: u64) -> u64 {
    unsafe { sys::register_len(reg_id) }
}

pub fn sha256_hash(input_reg: u64, output_reg: u64) {
    unsafe { sys::sha256(u64::MAX, input_reg, output_reg) }
}

pub fn value_return(input_reg: u64) {
    unsafe { sys::value_return(u64::MAX, input_reg) }
}

pub fn storage_write(key_register: u64, value_register: u64, eviction_register: u64) -> u64 {
    unsafe {
        sys::storage_write(
            u64::MAX,
            key_register,
            u64::MAX,
            value_register,
            eviction_register,
        )
    }
}

pub fn storage_write_from_reg(key_value: &[u8], value_register: u64) -> u64 {
    unsafe {
        sys::storage_write(
            key_value.len() as _,
            key_value.as_ptr() as _,
            u64::MAX,
            value_register,
            EVICTED_REGISTER,
        )
    }
}

pub fn storage_write_from_input(key_value: &[u8]) -> u64 {
    unsafe {
        sys::storage_write(
            key_value.len() as _,
            key_value.as_ptr() as _,
            u64::MAX,
            input(),
            EVICTED_REGISTER,
        )
    }
}

pub fn storage_read_from_reg_to_reg(key_register: u64, register_id: u64) -> Option<u64> {
    match unsafe { sys::storage_read(u64::MAX, key_register, register_id) } {
        1 => Some(register_id),
        _ => None,
    }
}

pub fn storage_read_from_reg(key_register: u64) -> Option<u64> {
    storage_read_from_reg_to_reg(key_register, Registers::StorageRead.into())
}

/// Read from storage using input register.
/// Returns Read register
pub fn storage_read_from_input() -> Option<u64> {
    storage_read_from_reg(input())
}

pub fn storage_read_to_reg(key: &[u8], into_reg: u64) -> Option<u64> {
    match unsafe { sys::storage_read(key.len() as _, key.as_ptr() as _, into_reg) } {
        1 => Some(into_reg),
        _ => None,
    }
}

pub fn storage_read(key: &[u8]) -> Option<u64> {
    storage_read_to_reg(key, Registers::StorageRead.into())
}

pub fn storage_has_key(key_register: u64) -> bool {
    unsafe { sys::storage_has_key(u64::MAX, key_register) != 0 }
}

fn use_reg<F: FnOnce(u64)>(reg: Registers, f: F) -> u64 {
    static mut READ_REGISTERS: [bool; 8] = [false; 8];
    let reg_int = reg.into();
    unsafe {
        if !READ_REGISTERS[reg_int as usize] {
            f(reg_int);
            READ_REGISTERS[reg_int as usize] = true;
        }
        reg_int
    }
}

// Context

pub fn input() -> u64 {
    use_reg(Registers::Input, |reg| unsafe { sys::input(reg) })
}

/// Returns the length of the input
pub fn input_len() -> u64 {
    env::register_len(input()).unwrap()
}

pub fn current_account_id() -> u64 {
    use_reg(Registers::CurrentAccountId, |reg_id| unsafe {
        sys::current_account_id(reg_id)
    })
}

pub fn predecessor_account_id() -> u64 {
    use_reg(Registers::PredecessorAccountId, |reg_id| unsafe {
        sys::predecessor_account_id(reg_id)
    })
}

// Promise

pub fn promise_batch_create(account_id_reg: u64) -> u64 {
    unsafe { sys::promise_batch_create(u64::MAX, account_id_reg) }
}

pub fn promise_batch_action_deploy_contract(promise_index: u64, bytes_reg: u64) -> u64 {
    unsafe {
        sys::promise_batch_action_deploy_contract(promise_index, u64::MAX, bytes_reg);
    };
    bytes_reg
}

pub fn promise_create(
    account_id_reg: u64,
    function_name: &str,
    args: &[u8],
    amount: u128,
    gas: u64,
) -> u64 {
    unsafe {
        sys::promise_create(
            u64::MAX,
            account_id_reg,
            function_name.len() as _,
            function_name.as_ptr() as _,
            args.len() as _,
            args.as_ptr() as _,
            &amount as *const u128 as _,
            gas,
        )
    }
}

pub fn promise_then(
    account_id_reg: u64,
    promise_index: u64,
    function_name: &str,
    args: &[u8],
    amount: u128,
    gas: u64,
) -> u64 {
    unsafe {
        sys::promise_then(
            promise_index,
            u64::MAX,
            account_id_reg,
            function_name.len() as _,
            function_name.as_ptr() as _,
            args.len() as _,
            args.as_ptr() as _,
            &amount as *const u128 as _,
            gas,
        )
    }
}

/// Returns register
pub fn promise_result(index: u64, reg_id: u64) -> u64 {
    match unsafe { sys::promise_result(index, reg_id) } {
        1 => reg_id,
        _ => env::panic_str("promise failed"),
    }
}

// Current account promises

pub fn promise_then_for_current(
    promise_index: u64,
    function_name: &str,
    args: &[u8],
    amount: u128,
    gas: u64,
) -> u64 {
    promise_then(
        current_account_id(),
        promise_index,
        function_name,
        args,
        amount,
        gas,
    )
}

pub fn promise_create_for_current(function_name: &str, args: &[u8], amount: u128, gas: u64) -> u64 {
    promise_create(current_account_id(), function_name, args, amount, gas)
}

pub fn promise_batch_create_for_current() -> u64 {
    promise_batch_create(current_account_id())
}

/// Returns promise_index
pub fn promise_batch_action_deploy_contract_for_current(bytes_reg: u64) -> u64 {
    promise_batch_action_deploy_contract(promise_batch_create_for_current(), bytes_reg)
}

pub fn promise_batch_create_for_predecessor() -> u64 {
    promise_batch_create(predecessor_account_id())
}
