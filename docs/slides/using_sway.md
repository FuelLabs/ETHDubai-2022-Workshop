### What is it like to use Sway?

---

`Forc`: the Sway toolchain

<ul class="size">
    <li>Project + dependency management system</li>
    <li>Equivalent of Rust's Cargo</li>
    <li>"Fuel Orchestrator"</li>
</ul>

---

<p class="size">
install:
</p>

```bash
$ cargo install forc fuel-core
```

<p class="size">
create a new Sway project:
</p>

```bash
$ forc init hello_world
$ cd hello_world
$ tree .
.
├── Cargo.toml
├── Forc.toml
├── src
│   └── main.sw
└── tests
    └── harness.rs
```

---

<p class="size">
Forc.toml:
</p>

<img src="./images/forc_toml2.png" height=200px />

<p class="size">
...btw has Sway has an extensive SDK 😊
</p>

Note:

* Sway's SDK is modeled after ether.js and ether.rs
* Compiling, deploying, and testing Sway contracts
* Launching a local Fuel network
* Generating type-safe Rust bindings of contract methods
* And more


---

<p class="size">
src/main.sw:
</p>

<img src="./images/test_contract.png" height=600px />

---

<p class="size">
start a Fuel full node:
</p>

```bash
$ fuel-core
Feb 15 13:29:40.508  INFO fuel_core::service::graph_api: Binding GraphQL provider to 127.0.0.1:4000
```

<p class="size">
build the project:
</p>

```bash
$ forc build
  Compiled library "core".
  Compiled library "lib-std".
  Compiled script "test_counter".
  Bytecode size is 248 bytes.
```

<p class="size">
deploy the contract:
</p>

```bash
$ forc deploy
Contract id: 0x9da075b76e379ac9276dfbd7d6270f5059270aae1679e64bae983b495e01d101
Logs:
HexString256(HexFormatted(0xb57db3004cad1b72157286b78ab904e30161964eff1caf56a1ae5cffa8e3bd83))
```

---

<p class="size">
test/harness.rs:
</p>

<img src="./images/test_contract_test.png" height=550px />

---

<p class="size">
test the project:
</p>

```bash
$ forc test
running 1 test
  Compiled library "core".
  Compiled library "lib-std".
  Compiled script "test_counter".
  Bytecode size is 248 bytes.
test harness ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.47s
```

<p class="size">
automatically format Sway code:
</p>

```bash
$ forc fmt
```