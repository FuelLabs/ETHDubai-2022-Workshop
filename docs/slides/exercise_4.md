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
* `ContractId`: ID of a contract. Wraps a `b256`.
* `Address`: a wallet address. Wraps a `b256`.
* `msg_sender()`: ID of the contract caller which could be an `Address` or a
  `ContractId`.
* `msg_asset_id()`: ID of the asset forwarded with the call.
* `msg_amount()`: amount of funds forwarded with the call.
* `this_balance(asset_id: ContractId)`: get the current contract's balance of
  coin `asset_id`.

---

#### ASSETS
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
* Use persistent storage to store the amount deposited at a slot. 
* A function for generating key is provided for you in the Github folder for
  Exercise 4.

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
* In `tests/harness.rs`, add and configure the boilerplate code from exercise 3.
* The SDK starts each wallet with some high default amount of `NATIVE_ASSET_ID`
  tokens.
* To deposit 10 coins of type `NATIVE_ASSET_ID`:

```rust
instance.deposit()
    .call_params(CallParameters::new(Some(10), None))
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
* To check the current balance of the wallet:

```rust
let wallet_coins = wallet.get_coins().await.unwrap();
```

---

Let's look at the other contract methods

--- 

#### ADDING LIQUIDITY
```rust
fn add_liquidity
    (min_liquidity: u64, max_tokens: u64, deadline: u64) -> u64;
```

* Given the current liquidity, the current reserve, and the deposited amount,
  reward the liquidity provider with some freshly minted liquidity tokens.

---

#### REMOVING LIQUIDITY
```rust
fn remove_liquidity
    (min_eth: u64, min_tokens: u64, deadline: u64) -> (u64, u64);
```
* Forward some liquidity tokens in exchange for an appropriate amounts of
both types of tokens. 

---

#### SWAP WITH MINIMUM
```rust
fn swap_with_minimum(min: u64, deadline: u64) -> u64;
```
* Forward a certain amount of tokens of one type in exchange for a minimum
  amount of the other token. 
* All forwarded tokens are exchanged.

---

#### SWAP WITH MAXIMUM
```rust
fn swap_with_maximum(amount: u64, deadline: u64) -> u64;
```
* Forward a certain amount of tokens of one type in exchange for a specific
  amount of the other token. 
* Any unspent token are transferred back to the sender.

---

Let's look at some pseudo-code.
