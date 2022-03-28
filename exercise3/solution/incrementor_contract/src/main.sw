contract;

use std::storage::{get, store, hash::{hash_u64, HashMethod}};
use incrementor_abi::Incrementor;

const KEY = 0x0000000000000000000000000000000000000000000000000000000000000000;

impl Incrementor for Contract {
    fn initialize(initial_value: u64) {
        store(KEY, initial_value);
    }

    fn increment(increment_by: u64) {
        let incremented = get::<u64>(KEY) + increment_by;
        store(KEY, incremented);
    }

    fn get() -> u64 {
        get::<u64>(KEY)
    }
}
