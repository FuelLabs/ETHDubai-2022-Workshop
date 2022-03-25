script;

use swayswap_abi::Exchange;
use std::{chain::assert, contract_id::ContractId};

fn main() {
    let caller = abi(Exchange, 0x55d474d61cb09ca053fe43a4a0a0755eec11d72c9807f932d3b7e2cf9a1f9e87);

    caller.deposit{
        coins: 50,
        asset_id:  0x55d474d61cb09ca053fe43a4a0a0755eec11d72c9807f932d3b7e2cf9a1f9e87,
    }();

//    caller.withdraw(0, ~ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000001));
}
