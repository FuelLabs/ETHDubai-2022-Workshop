script;

use incrementor_abi::Incrementor;

use std::chain::assert;

fn main() {
    let abi = abi(Incrementor,  0xbe31fb6902be44514b09ef3ccbb7c49370add629e8d0a29946ffbecf49688f8a);
    
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
