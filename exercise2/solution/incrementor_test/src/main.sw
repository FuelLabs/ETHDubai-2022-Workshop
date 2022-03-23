script;

use incrementor_abi::Incrementor;

use std::chain::assert;

fn main() {
    let abi = abi(Incrementor, 0x8cf7518116e0886cfc33222547b5d0198071841b11e0a7c4f3d3e42d0d91e221);
    
    // Initialize locations 0 and 1 to 0
    abi.initialize {
        gas: 10000
    }
    (0, 0);
    abi.initialize {
        gas: 10000
    }
    (1, 0);
    assert(abi.get{gas: 10000}(0) == 0);
    assert(abi.get{gas: 10000}(1) == 0);

    // Increment location 0 twice
    abi.increment {
        gas: 10000
    }
    (0, 2);
    abi.increment {
        gas: 10000
    }
    (0, 5);
    assert(abi.get{gas: 10000}(0) == 7);

    // Increment location 1 twice
    abi.increment {
        gas: 10000
    }
    (1, 4);
    abi.increment {
        gas: 10000
    }
    (1, 9);
    assert(abi.get{gas: 10000}(1) == 13);
}
