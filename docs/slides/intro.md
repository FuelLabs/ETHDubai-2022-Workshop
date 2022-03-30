The material for this workshop is available at
https://github.com/FuelLabs/ETHDubai-2022-Workshop

---

#### INSTALL THE DEPENDENCIES

* Steps in https://fuellabs.github.io/sway/
  * Install Rust using the steps described in the link provided:
    https://www.rust-lang.org/tools/install
  * Install other dependencies:

```bash
# MacOS
brew update
brew install openssl cmake llvm libpq
```

```bash
# Debian 
apt update
apt install -y cmake pkg-config libssl-dev git gcc 
               build-essential git clang libclang-dev 
               llvm libpq-dev
```

---

#### INSTALL THE TOOLCHAIN
```bash
cargo install forc fuel-core
```
* `forc` is the equivalent of `Cargo` but for `Sway`.
  * It is the primary entry point for creating, building, testing and deploying
    Sway projects.
  * Run 
```bash
forc --version
``` 
to make sure that `forc` is built correctly.

---

#### IDE
* You're welcome to use an IDE of your choice.
  * A VSCode plugin for Sway is available:
    https://marketplace.visualstudio.com/items?itemName=FuelLabs.sway-vscode-plugin
  * Syntax highlighting for Vim is also available:
    https://github.com/FuelLabs/sway.vim
