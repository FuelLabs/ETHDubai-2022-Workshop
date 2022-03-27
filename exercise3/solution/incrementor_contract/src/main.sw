contract;

use std::storage::{get, store, hash::{hash_u64, HashMethod}};
use incrementor_abi::Incrementor;

impl Incrementor for Contract {
    fn initialize(key: u64, initial_value: u64) {
        let storage_key = hash_u64(key, HashMethod::Sha256);
        store(storage_key, initial_value);
    }

    fn increment(key: u64, increment_by: u64) {
        let storage_key = hash_u64(key, HashMethod::Sha256);
        let incremented = get::<u64>(storage_key) + increment_by;
        store(storage_key, incremented);
    }

    fn get(key: u64) -> u64 {
        let storage_key = hash_u64(key, HashMethod::Sha256);
        get::<u64>(storage_key)
    }
}
