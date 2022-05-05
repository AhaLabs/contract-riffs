use near_sdk::sys;

pub const EVICTED_REGISTER: u64 = std::u64::MAX - 1;
pub const DATA_REGISTER: u64 = std::u64::MAX - 2;

pub fn input(reg_id: u64) -> u64 {
    unsafe { sys::input(reg_id) };
    reg_id
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

pub fn storage_read(key_register: u64, eviction_reg: u64) -> Option<u64> {
    match unsafe { sys::storage_read(u64::MAX, key_register, eviction_reg) } {
        1 => Some(eviction_reg),
        _ => None,
    }
}

pub fn storage_has_key(key_register: u64) -> bool {
    unsafe { sys::storage_has_key(u64::MAX, key_register) != 0 }
}

pub fn promise_batch_create(account_id_reg: u64) -> u64 {
    unsafe { sys::promise_batch_create(u64::MAX, account_id_reg) }
}
