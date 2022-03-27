### EXERCISE 2

Let's write our first contract.

---

#### SIMPLE COUNTER
* We will implement an "incrementor": starting with some initial value,
  increment that value by a certain amount each time the contract is called.

* We will use two new types of programs:
  * Library
  * Contract

* We will also use a script to call the contract.

* We will use a library to define an ABI for our contract.

---

#### CONTRACT ABI
* In a fresh project, modify `src/main.sw` to define our contract ABI as a
  library:

```rust
library incrementor_abi;

abi Incrementor {
    fn initialize(initial_value: u64);
    fn increment(incrementor_by: u64);
    fn get() -> u64;
}
```

---

#### CONTRACT DEPENDENCIES 
* In another fresh project, modify `Forc.toml` to make `incrementor_abi` a
  dependency for the contract:
```toml
[dependencies]
std = { git = "https://github.com/FuelLabs/sway-lib-std" }
incrementor_abi = { path = "/path/to/incrementor_abi" }
```
* This allows use to start importing symbols from the library
  `incrementor_abi`. Specifically, we want the ABI `Incrementor`.
```
use incrementor_abi::Incrementor;
```

---

#### CONTRACT IMPLEMENTATION 
* Change `src/main.sw` from a script to a contract, import the ABI
  `Incrementor`, and create an `impl` block to implement the methods of the
  ABI:

```rust
use incrementor_abi::Incrementor;

impl Incrementor for Contract {
    fn initialize(key: u64, initial_value: u64) { ... }
    fn increment(key: u64, increment_by: u64) { ... }
    fn get(key: u64) -> u64 { ... }
}
```

---

#### A NOTE ON  STORAGE
* To keep track of the value to increment, we need persistent storage.
* Persistent storage can be declared using the `storage` keyword:

```rust
storage {
    field_1: some_type,
    field_2: some_type,
    ...
}
```

A storage field `x` can be read and written using:
```rust
... = storage.x;
storage.x = ...;
```

---

#### STORAGE EXAMPLE
```rust
storage {
    val: u64
}

impl Incrementor for Contract {
    fn get() -> u64 { storage.val }
    fn set(new_val: u64) { storage.val = new_val; }
}
```

---

#### PROCEED WITH YOUR CONTRACT IMPLEMENTATION
* Once you've implemented all the contract methods, you're
ready to deploy youre contract.
* Make sure you have `fuel-core` running and then run:
```bash
$ forc deploy
...
Contract id: 0x1534.....
Logs:
TransactionId(HexFormatted(d789.....))
```
* Copy the resulting contract ID and keep it somewhere. You will need it later.
* The contract is now ready to be called!

---

#### CALLING YOUR CONTRACT FROM A SCRIPT
* In a third fresh project, create a dependency on `incrementor_abi` and import
  the ABI `Incrementor`.
* To start calling our contract methods, we need a contract caller object which
  we can instantiate as follows:
```rust
let caller = abi(Incrementor, <contract_id>);
```
* We can now start calling our contract methods using the caller as follows:
```
caller.initialize(..);
```

---

#### TEST YOUR CONTRACT
* In `main()`, call your contract methods successively and check the results using `assert`. 
