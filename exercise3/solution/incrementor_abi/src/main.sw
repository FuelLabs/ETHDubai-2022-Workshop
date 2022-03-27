library incrementor_abi;

abi Incrementor {
    fn initialize(key: u64, initial_value: u64);
    fn increment(key: u64, initial_value: u64);
    fn get(key: u64) -> u64;
}
