// use near_sdk::{
//     borsh::{self, BorshDeserialize, BorshSerialize},
//     env,
//     near_bindgen,
//     require,
//     sys,
//     witgen,
//     // collections::{TreeMap, LazyOption, UnorderedMap},
//     AccountId,
//     Balance,
// };

// // #[derive(BorshDeserialize, BorshSerialize)]
// // struct Binary {
// //   hash: [u8; 32],
// //   wit: LazyOption<String>,
// // }

// #[near_bindgen]
// #[derive(BorshDeserialize, BorshSerialize, Default)]
// pub struct Contract {
//     // contracts: UnorderedMap<AccountId, TreeMap<Version, Option<Binary>>>,
// }

// #[near_bindgen]
// impl Contract {
//     #[payable]
//     pub fn create_account_and_deploy(&mut self, new_account_id: &AccountId) {}

//     // pub fn deploy_and_init(&)
// }

// fn input(registry_id: u64) {
//     unsafe { sys::input(registry_id) }
// }

// fn sha256_hash(registry_id: u64) -> u64 {
//     let next_id = registry_id + 1;
//     unsafe { sys::sha256(u64::MAX, registry_id, next_id) };
//     next_id
// }

// fn value_return(register_id: u64) {
//     unsafe { sys::value_return(u64::MAX, register_id) }
// }

// fn storage_write(key_register: u64, value_register: u64, eviction_register: u64) -> u64 {
//     unsafe {
//         sys::storage_write(
//             u64::MAX,
//             key_register,
//             u64::MAX,
//             value_register,
//             eviction_register,
//         )
//     }
// }

// fn storage_read(key_register: u64) -> Option<u64> {
//     let eviction_reg = key_register + 1;
//     match unsafe { sys::storage_read(u64::MAX, key_register, eviction_reg) } {
//         1 => Some(eviction_reg),
//         _ => None,
//     }
// }

// fn storage_has_key(key_register: u64) -> bool {
//     unsafe { sys::storage_has_key(u64::MAX, key_register) != 0 }
// }

// fn promise_batch_action_function_call_fetch(promise_index: u64, function_name: &str, gas: u64) {
//     let amount = 0u128;

//     unsafe {
//         sys::input(3);
//         sys::promise_batch_action_function_call(
//             promise_index,
//             function_name.len() as _,
//             function_name.as_ptr() as _,
//             u64::MAX,
//             3u64,
//             &amount as *const Balance as _,
//             gas,
//         )
//     }
// }

// fn promise_batch_then(promise_index: u64, account_id: &[u8; 64]) -> u64 {
//     unsafe { sys::promise_batch_then(promise_index, 64, account_id.as_ptr() as _) }
// }

// fn promise_batch_create(account_id: &[u8; 64]) -> u64 {
//     unsafe { sys::promise_batch_create(account_id.len() as _, account_id.as_ptr() as _) }
// }

// fn promise_batch_action_function_call(
//     promise_index: u64,
//     method_name: &str,
//     arguments: &[u8],
//     amount: Balance,
//     gas: u64,
// ) {
//     unsafe {
//         sys::promise_batch_action_function_call(
//             promise_index,
//             method_name.len() as _,
//             method_name.as_ptr() as _,
//             arguments.len() as _,
//             arguments.as_ptr() as _,
//             &amount as *const Balance as _,
//             gas,
//         )
//     }
// }
