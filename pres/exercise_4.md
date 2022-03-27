### EXERCISE 4

Let's implement Swayswap!

---

#### Swayswap
* We're going to implement a clone of "Uniswap v1" in Sway.
* Our contract will have the following methods:

```rust
/// Deposit coins for later adding to liquidity pool.
fn deposit();
/// Withdraw coins that have not been added to a liquidity pool yet.
fn withdraw(amount: u64, asset_id: ContractId);
/// Deposit ETH and Tokens at current ratio to mint SWAYSWAP tokens.
fn add_liquidity(min_liquidity: u64, max_tokens: u64, deadline: u64) -> u64;
/// Burn SWAYSWAP tokens to withdraw ETH and Tokens at current ratio.
fn remove_liquidity(min_eth: u64, min_tokens: u64, deadline: u64) -> (u64, u64);
/// Swap ETH <-> Tokens and tranfers to sender.
fn swap_with_minimum(min: u64, deadline: u64) -> u64;
/// Swap ETH <-> Tokens and tranfers to sender.
fn swap_with_maximum(amount: u64, max: u64, deadline: u64) -> u64;
```

---

Let's go over some useful objects and methods from the standard libraries that
we're going to need for Swayswap.

---

#### ADDRESS AND CONTRACTID
* `ContractId`: ID of a deployd contract. Wraps a `b256`.
* `Address`: a wallet address. Wraps a `b256`.
* `msg_sender()`: returns the ID of the contract caller which could be an
  `Address` or a `ContractId`.
* `msg_asset_id()`: returns the ID of the native asset forwarded with the
  contract.
* `this_balance(asset_id: ContractId)`: get the current contract's balance of
  coin `asset_id`.

---

#### NATIVE ASSETS
* `mint(amount: u64)`: mint some coins of the current contract's asset ID,
  which is basically the contract ID itself.
* `burn(amount: u64)`: burn some coins of the current contract's asset ID.

---

#### TRANSFERS
* Transfer some amount of coins of type `asset_id` to some `Address`:
```rust
transfer_to_output
        (amount: u64, asset_id: ContractId, recipient: Address)
```

---

#### OVERVIEW 
* Our exchange will support two asset IDs: `NATIVE_ASSET_ID` and `TOKEN_ID` and
  will allow us to swap between them.
* Our contract will reward liquidity providers with its own token.
* A liquidity provider has to deposit both types of assets (using `deposit()`)
  before they can call `add_liquidity()` which will reward them with the
  contract tokens.

---

#### DEPOSIT
```rust
fn deposit();
```
* Make sure that the coins forwarded are one of the supported token types.
* Use persistent storage to store the amount deposited at a slot that depends
  on three parameters (use `hash_pair`):
  * The asset ID of the coins forwarded.
  * The address of the sender.
  * An identifier to indicate that this slot is for deposits.

---

#### WITHDRAW
```rust
withdraw(amount: u64, asset_id: ContractId);
```
* Make sure that the coins forwarded are one of the supported token types.
* Look up whether the sender has enough deposited coins of type `asset_id`.
* If so, update the storage slot with the remaining balance and then call
  `transfer_to_output`.

---

Before we continue, let's test what we have so far.

---

#### CALLING DEPOSIT()
* In `tests/harness.rs`, add and configure the boilerplate code from exercise
  3.
* The SDK starts each wallet with a default amount of 100 `NATIVE_ASSET_ID` tokens.
* To deposit 10 coins of type `NATIVE_ASSET_ID`:

```rust
let call_params = CallParameters::new(Some(10), None);
instance.deposit()
    .call_params(call_params)
    .call()
    .await
    .unwrap();
}
```

---

#### CALLING WITHDRAW()
* To withdraw 6 coins of type `NATIVE_ASSET_ID`:
```rust
instance.withdraw(10, AssetId::from(NATIVE_ASSET_ID))
    .append_variable_outputs(1)
    .call()
    .await
    .unwrap();
```
* You may have to import `NATIVE_ASSET_ID` first:
```rust
use fuels_core::constants::NATIVE_ASSET_ID;
```

---

#### VERIFY
* Running `cargo test` should now pass. 
* If you try to withdraw more funds than available, the test should fail.
* If you want to check the remaining balance, you can add a contract method
  `get_my_balance()` which will fetch the remaining balance for the wallet in
  the contract.

---

#### ADD LIQUIDITY
```rust
fn add_liquidity
    (min_liquidity: u64, max_tokens: u64, deadline: u64) -> u64;
```
* Goal is to have the contract mint liquidity tokens and send them to the
  liquidity provider, according to some formula.
* Assumption is that the sender already deposited both types of coins before
  calling `add_liquidity`. 




