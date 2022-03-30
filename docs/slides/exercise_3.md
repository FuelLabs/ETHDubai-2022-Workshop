### EXERCISE 3

Let's test our contract using the Rust SDK

---

* Instead of testing our contract with a script, we will write a proper test
  using Fuel's Rust SDK.
* An SDK test usually starts as follows:

```rust
use fuel_tx::Salt;
use fuels_abigen_macro::abigen;
use fuels_contract::{contract::Contract, parameters::TxParameters};
use fuels_signers::util::test_helpers;

abigen!(IncTest, "out/debug/incrementor_contract-abi.json");

#[tokio::test]
async fn mint() {
    let salt = Salt::from([0u8; 32]);
    let compiled =
        Contract::load_sway_contract("out/debug/incrementor_contract.bin", salt)
            .unwrap();

    let (provider, wallet) = test_helpers::setup_test_provider_and_wallet().await;
    let id = Contract::deploy(&compiled, &provider, &wallet, TxParameters::default())
        .await
        .unwrap();

    let instance = IncTest::new(id.to_string(), provider, wallet);
}
```

---

#### Imports

```rust
use fuel_tx::Salt;
use fuels_abigen_macro::abigen;
use fuels_contract::{contract::Contract, parameters::TxParameters};
use fuels_signers::util::test_helpers;
```

#### ABIGEN

```rust
abigen!(IncTest, "out/debug/incrementor_contract-abi.json");
```

#### TEST 
```rust
#[tokio::test]
async fn mint() {
   ... 
}
```

---

#### TEST BODY

```rust
let salt = Salt::from([0u8; 32]);
let compiled =
    Contract::load_sway_contract
        ("out/debug/incrementor_contract.bin", salt).unwrap();

let (provider, wallet) = 
    test_helpers::setup_test_provider_and_wallet().await;

let id = 
    Contract::deploy
        (&compiled, &provider, &wallet, TxParameters::default())
    .await
    .unwrap();

let instance = IncTest::new(id.to_string(), provider, wallet);
```

---

That's all we need to start calling contracts!

---

#### CALLING CONTRACT METHODS
* We can now use the object `instance` to start calling the contract methods
  available to us:

```
instance.initialize(0, 0)
    .call()
    .await
    .unwrap();
```

```
let result = instance.get(0, 0)
    .call()
    .await
    .unwrap();
```

---

#### VALIDATION
* To run your test, you need to build your contract and then run `cargo test`.

```bash
$ forc build
  Compiled library "core".
  Compiled library "std".
  Compiled library "incrementor_abi".
  Compiled contract "incrementor_contract".
  Bytecode size is 500 bytes.
```

```bash
$ cargo test
  ...
running 1 test
test mint ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.08s
```
