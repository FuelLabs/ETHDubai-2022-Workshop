### EXERCISE 0 

---

Create a new empty Sway project

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

Inspect `src/main.sw`

```rust
script;

fn main() {

}
```

---

Inspect `Forc.toml`

```toml
[project]
authors = ["Mohammad"]
entry = "main.sw"
license = "Apache-2.0"
name = "hello_world"

[dependencies]
std = { git = "https://github.com/FuelLabs/sway", branch = "master" }
```

---

Build the project

```bash
$ forc build
  ...
  Compiled library "std".
  Compiled script "hello_world".
  Bytecode size is 28 bytes.
```

---

Start a local Fuel full node

```bash
$ fuel-core --db-type=in-memory
March 29 13:29:40.508  INFO fuel_core::service::graph_api: Binding GraphQL provider to 127.0.0.1:4000
```

Run the script on the node

```bash
$ forc run --pretty-print
  ...
  Bytecode size is 28 bytes.
  [
      Return {
          id: 0x0000000000000000000000000000000000000000000000000000000000000000,
          val: 0,
          pc: 496,
          is: 472,
      },
      ScriptResult {
          result: InstructionResult {
              reason: RESERV00,
              instruction: Instruction {
                  op: 0,
                  ra: 0,
                  rb: 0,
                  rc: 0,
                  rd: 0,
                  imm06: 0,
                  imm12: 0,
                  imm18: 0,
                  imm24: 0,
              },
          },
          gas_used: 44,
      },
  ]
```
