### EXERCISE 1

Let's write and test a `check_prime()` function in Sway.

---

#### Primality Checker

* In a fresh Sway script, write a simple function `check_prime` that takes an
  integer `n` and returns `true` if `n` is prime and `false` otherwise.
```rust
fn check_prime(n: u64) -> bool {
    ...
}
```
* Use the Sway Book https://fuellabs.github.io/sway/ (or ask me!) if you need
  help with the syntax.
* If you've written Rust before, the Sway syntax should look quite familiar to
  you.

---

#### Testing
* To test your function, import `assert` from the standard library and use it
  to check the results.
```rust
use std::assert::assert;
```
* In `main()`, call your `check_prime()` function and use
  `assert` to check the result:
```rust
assert(check_prime(11));
assert(!check_prime(42));
```
* After adding enough asserts, run your project using `forc run` and inspect
  the output.
* If any of the asserts fails, you will see a `Revert` in the output.
