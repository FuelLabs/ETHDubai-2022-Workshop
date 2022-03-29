### Swayswap
Write a clone of Uniswap v1 using Sway and test it using the Rust SDK.

Pricing functions:

```rust
/// Pricing function for converting between ETH and Token when buying. 
/// To be used for swap_with_minimum().
fn get_input_price(input_amount: u64, input_reserve: u64, output_reserve: u64) -> u64 {
    assert(input_reserve > 0 && output_reserve > 0);
    let input_amount_with_fee: u64 = input_amount * 997;
    let numerator: u64 = input_amount_with_fee * output_reserve;
    let denominator: u64 = (input_reserve * 1000) + input_amount_with_fee;
    numerator / denominator
}

/// Pricing function for converting between ETH and Tokens when selling.
/// To be used for swap_with_maximum().
fn get_output_price(output_amount: u64, input_reserve: u64, output_reserve: u64) -> u64 {
    assert(input_reserve > 0 && output_reserve > 0);
    let numerator: u64 = input_reserve * output_reserve * 1000;
    let denominator: u64 = (output_reserve - output_amount) * 997;
    numerator / denominator + 1
}
```

Helper functions:

```
/// Compute the storage slot for an address's deposits.
fn key_deposits(a: Address, asset_id: b256) -> b256 {
    const S_DEPOSITS: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000;
    let inner = hash_pair(a.into(), asset_id, HashMethod::Sha256);
    hash_pair(S_DEPOSITS, inner, HashMethod::Sha256)
}

/// Return the sender as an Address or panic
fn get_msg_sender_address_or_panic() -> Address {
    let result: Result<Sender, AuthError> = msg_sender();
    let mut ret = ~Address::from(0x0000000000000000000000000000000000000000000000000000000000000000);
    if result.is_err() {
        panic(0);
    } else {
        let unwrapped = result.unwrap();
        if let Sender::Address(v) = unwrapped {
            ret = v;
        } else {
            panic(0);
        };
    };

    ret
}
```

Here's the the contract in pseudo-code:

```rust
deposit() {
    Check msg_asset_id()
    Get the sender which we expect to be an Address
    Update the new amount in deposit storage for Address
}

withdraw(amount, asset_id) {
    Check msg_asset_id()
    Get the sender which we expect to be an Address
    Update the new amount in deposit storage for Address
    Call transfer_to_output() to move funds from the contract to the Address
}

add_liquidity(min_liquidity, max_tokens, deadline) -> u64 {
    // No coins should be sent with this call. Coins should instead be `deposit`ed prior.
    Check msg_asset_id()
    Check that msg_amount() is zero
    Check that the deadline is met
    Check max_token
    Get the sender which we expect to be an Address
    
    total_liquidity = the current total liquidity from storage
    eth_amount = eth amount stored for Address;
    current_token_amount = token amount stored for Address;

    Drain all eth from Address's account.

    if total_liquidity > 0 {
        eth_reserve = total amount of eth in this contract;
        token_reserve = total amount of token in this contract;
        token_amount = (eth_amount * token_reserve) / eth_reserve;
        liquidity_minted = (eth_amount * total_liquidity) / eth_reserve;
        
        mint(liquidity_minted);

        Update the total liquidity supply 
        Call transfer_to_output() to move liquidity tokens from the contract to the Address
        Update the token amount stored for Address after deducting token_amount
    } else {
        initial_liquidity = total amount of eth in this contract;

        mint(initial_liquidity);
        
        Update the total liquidity supply 
        Call transfer_to_output() to move liquidity tokens from the contract to the Address
        Update the token amount stored for Address after deducting max_token
    };

    Return the amount of liquidity tokens minted
}

fn remove_liquidity(min_eth, min_tokens, deadline) -> RemoveLiquidityReturn {
    Check msg_amount() and msg_asset_id()
    Check that the forwarded funds are liquidity tokens  
    Check that the deadline is met
    Get the sender which we expect to be an Address
    
    total_liquidity = current total liquidity from storage;

    eth_reserve = total amount of eth in this contract
    token_reserve = total amount of tokens in this contract

    eth_amount = msg_amount() * eth_reserve / total_liquidity;
    token_amount = msg_amount() * token_reserve / total_liquidity;

    Check eth_amount and token_amount v.s. min_eth and min_token

    burn(msg_amount());

    Update the new total liquidity token supply 
    Call transfer_to_output() to move eth_amount from the contract to Address
    Call transfer_to_output() to move token_amount from the contract to Address

    Return RemoveLiquidityReturn{ eth_amount: eth_amount, token_amount: token_amount }
}

fn swap_with_minimum(min, deadline) -> u64 {
    Check that the deadline is met
    Check msg_amount() and msg_asset_id()
    Get the sender which we expect to be an Address

    eth_reserve = total amount of eth in this contract
    token_reserve = total amount of tokens in this contract

    if (eth is forwarded) {
        tokens_bought = get_input_price(msg_amount(), eth_reserve, token_reserve);
        Check that the amount of tokens bought satisfy the min
        Move tokens_bought to Address
    } else { // if token is forwarded
        eth_bought = get_input_price(msg_amount(), token_reserve, eth_reserve);
        Check that the amount of eth bought satisfy the min
        Move eth_bought to Address
    }
    
    Return the amount bought
}

fn swap_with_maximum(amount, deadline) -> u64 {
    Check that the deadline is met
    Check msg_amount()
    Check msg_asset_id()
    Get the sender which we expect to be an Address

    eth_reserve = total amount of eth in this contract
    token_reserve = total amount of tokens in this contract

    if (eth is forwarded) {
        eth_sold = get_output_price(amount, eth_reserve, token_reserve);
        refund = msg_amount() - eth_sold;
        if refund > 0 {
            Move refund (eth) to Address
        }
        Move amount (token) to Address
    } else {
        tokens_sold = get_output_price(amount, token_reserve, eth_reserve);
        refund = msg_amount() - tokens_sold;
        if refund > 0 {
            Move refund (token) to Address 
        }
        Move amount (eth) to Address
    }

    Return the amount sold
}
