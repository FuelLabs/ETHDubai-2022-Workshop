script;

use incrementor_abi::Incrementor;

use std::chain::assert;

fn main() {
    let caller = abi(Incrementor,  0xbe31fb6902be44514b09ef3ccbb7c49370add629e8d0a29946ffbecf49688f8a);
    
    caller.initialize(1);
    assert(caller.get() == 1);

    // Increment location 0 twice
    caller.increment(2);
    caller.increment(5);
    assert(caller.get() == 8);
}
