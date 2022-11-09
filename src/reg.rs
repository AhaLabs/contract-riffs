use near_sdk::{env, require, sys};

const EVICTED: u64 = u64::MAX - 1;
// const DATA: u64 = u64::MAX - 2;
pub enum Registers {
    Input = 0,
    CurrentAccountId = 1,
    PredecessorAccountId = 2,
    SignerAccountId = 3,
    SignerAccountPk = 4,
    StorageRead = 5,
    StorageWriteEviction = 6,
    PromiseResult0 = 7,
    PromiseResult1 = 8,
    PromiseResult2 = 9,
    PromiseResult3 = 10,
}

impl Registers {
    pub fn use_reg<F: FnOnce(u64)>(self, f: F) -> u64 {
        static mut READ_REGISTERS: [bool; 11] = [false; 11];
        let reg_int = self.into();
        unsafe {
            if !READ_REGISTERS[reg_int as usize] {
                f(reg_int);
                READ_REGISTERS[reg_int as usize] = true;
            }
            reg_int
        }
    }

    pub fn from_promise_index(index: u64) -> Self {
        require!(index < 4, "promise index cannot be greater than 3 {}");
        match index {
            0 => Registers::PromiseResult0,
            1 => Registers::PromiseResult1,
            2 => Registers::PromiseResult2,
            3 => Registers::PromiseResult3,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<u64> for Registers {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Registers::Input,
            1 => Registers::CurrentAccountId,
            2 => Registers::PredecessorAccountId,
            3 => Registers::SignerAccountId,
            4 => Registers::SignerAccountPk,
            5 => Registers::StorageRead,
            6 => Registers::StorageWriteEviction,
            7 => Registers::PromiseResult0,
            8 => Registers::PromiseResult1,
            9 => Registers::PromiseResult2,
            10 => Registers::PromiseResult3,
            _ => return Err("invalid range for register"),
        })
    }
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

pub fn storage_write_from_reg(key_register: u64, value_register: u64) -> u64 {
    unsafe { sys::storage_write(u64::MAX, key_register, u64::MAX, value_register, EVICTED) }
}

pub fn storage_write(key_value: &[u8], value_register: u64) -> u64 {
    unsafe {
        sys::storage_write(
            key_value.len() as _,
            key_value.as_ptr() as _,
            u64::MAX,
            value_register,
            EVICTED,
        )
    }
}

pub fn storage_write_input(key_value: &[u8]) -> u64 {
    storage_write(key_value, input())
}

pub fn storage_write_input_from_reg(key_register: u64) -> u64 {
    storage_write_from_reg(key_register, input())
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

// Context

pub fn input() -> u64 {
    Registers::Input.use_reg(|reg| unsafe { sys::input(reg) })
}

/// Returns the length of the input
pub fn input_len() -> u64 {
    env::register_len(input()).unwrap()
}

pub fn input_is_empty() -> bool {
    input_len() == 0
}

pub fn signer_account_pk() -> u64 {
    Registers::SignerAccountPk.use_reg(|reg| unsafe { sys::signer_account_pk(reg) })
}

pub fn current_account_id() -> u64 {
    Registers::CurrentAccountId.use_reg(|reg_id| unsafe { sys::current_account_id(reg_id) })
}

pub fn predecessor_account_id() -> u64 {
    Registers::PredecessorAccountId.use_reg(|reg_id| unsafe { sys::predecessor_account_id(reg_id) })
}

// Promise

pub fn promise_batch_create_from_reg(account_id_reg: u64) -> u64 {
    unsafe { sys::promise_batch_create(u64::MAX, account_id_reg) }
}

pub fn promise_batch_action_deploy_contract(promise_index: u64, bytes_reg: u64) -> u64 {
    unsafe {
        sys::promise_batch_action_deploy_contract(promise_index, u64::MAX, bytes_reg)
    };
    promise_index
}

pub fn promise_batch_action_delete_key(promise_index: u64, public_key_reg: u64) -> u64 {
    unsafe {
        sys::promise_batch_action_delete_key(promise_index, u64::MAX, public_key_reg);
    };
    promise_index
}

pub fn promise_batch_action_delete_key_of_signer(promise_index: u64) -> u64 {
    promise_batch_action_delete_key(promise_index, signer_account_pk())
}

pub fn promise_create_account_from_reg(
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

pub fn promise_create_args_from_reg(
    account_id: &str,
    function_name: &str,
    args_reg_id: u64,
    amount: u128,
    gas: u64,
) -> u64 {
    unsafe {
        sys::promise_create(
            account_id.len() as _,
            account_id.as_ptr() as _,
            function_name.len() as _,
            function_name.as_ptr() as _,
            u64::MAX,
            args_reg_id,
            &amount as *const u128 as _,
            gas,
        )
    }
}

/// Create a promise where the arguments are from input
pub fn promise_create_args_from_input(
    account_id: &str,
    function_name: &str,
    amount: u128,
    gas: u64,
) -> u64 {
    promise_create_args_from_reg(account_id, function_name, input(), amount, gas)
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

/// Will get the promise result at index and place it in a register
/// Currently index must be less than or equal to the 4
pub fn promise_result(index: u64) -> u64 {
    Registers::from_promise_index(index).use_reg(|reg_id| {
        match unsafe { sys::promise_result(index, reg_id) } {
            1 => (),
            _ => env::panic_str("promise failed"),
        }
    })
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
    promise_create_account_from_reg(current_account_id(), function_name, args, amount, gas)
}

/// Create a promise batch for the current executing contract
pub fn promise_batch_create_for_current() -> u64 {
    promise_batch_create_from_reg(current_account_id())
}

/// Create a promise batch action for current account for deploying a contract found in `bytes_reg`
pub fn promise_batch_action_deploy_contract_for_current(bytes_reg: u64) -> u64 {
    promise_batch_action_deploy_contract(promise_batch_create_for_current(), bytes_reg)
}

pub fn promise_batch_create_for_predecessor() -> u64 {
    promise_batch_create_from_reg(predecessor_account_id())
}
