use contract_utils::{
    near_sdk::{require, witgen},
    refund_storage_cost, reg,
};

#[no_mangle]
pub fn upload() {
    _upload().unwrap()
}

fn _upload() -> Option<()> {
    let input_id = 0;
    reg::input(input_id);
    let sha_id = 1;
    reg::sha256_hash(input_id, sha_id);
    require!(!reg::storage_has_key(sha_id), "ERR_ALREADY_EXISTS");
    let eviction_id = sha_id + 1;
    refund_storage_cost(
        || {
            reg::storage_write(input_id, sha_id, eviction_id);
        },
        eviction_id + 1,
    );
    reg::value_return(sha_id);
    Some(())
}

#[no_mangle]
pub fn fetch() {
    reg::input(0);
    reg::value_return(reg::storage_read(0, 1).expect("MISSING BINARY"));
}

#[allow(dead_code, unused_variables)]
mod private {
    use super::witgen;
    /// Stores the bytes at its corresponding sha256 hash
    /// change
    #[witgen]
    pub fn upload() -> Vec<u8> {
        vec![]
    }

    /// Fetch binary corresponding the sha256
    #[witgen]
    pub fn fetch() -> Vec<u8> {
        vec![]
    }
}
