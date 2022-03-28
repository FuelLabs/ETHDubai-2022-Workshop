use fuel_tx::{AssetId, Salt};
use fuels_abigen_macro::abigen;
use fuels_contract::{
    contract::Contract,
    parameters::{CallParameters, TxParameters},
};
use fuels_core::constants::NATIVE_ASSET_ID;
use fuels_signers::{util::test_helpers, Signer};

#[tokio::test]
async fn swayswap() {
    // Provider and Wallet
    let (provider, wallet) = test_helpers::setup_test_provider_and_wallet().await;

    /////////////////////////////
    // Load the Swayswap contract
    /////////////////////////////
    abigen!(TestSwayswap, "out/debug/swayswap_contract-abi.json");
    let swayswap_salt = Salt::from([0u8; 32]);
    let swayswap_compiled =
        Contract::load_sway_contract("out/debug/swayswap_contract.bin", swayswap_salt).unwrap();

    // Get the contract ID and a handle to it
    let swayswap_contract_id = Contract::deploy(
        &swayswap_compiled,
        &provider.clone(),
        &wallet.clone(),
        TxParameters::default(),
    )
    .await
    .unwrap();
    let swayswap_instance = TestSwayswap::new(
        swayswap_contract_id.to_string(),
        provider.clone(),
        wallet.clone(),
    );

    // Depost some native assets
    let call_params = CallParameters::new(Some(11), None);
    swayswap_instance
        .deposit()
        .call_params(call_params)
        .call()
        .await
        .unwrap();

    // Withdraw some native assets
    let native_asset_id = testswayswap_mod::ContractId {
        value: NATIVE_ASSET_ID,
    };
    swayswap_instance
        .withdraw(9, native_asset_id)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();

    let wallet_native_asset_coins = wallet.get_coins().await.unwrap();

    // This assert should pass when `msg_sender()` is used in Swayswap
    // asserteq!(wallet_native_asset_coins, 100 - 11 + 9);

    //////////////////////////
    // Load the Token contract
    //////////////////////////
    abigen!(
        TestToken,
        "../token_contract/out/debug/token_contract-abi.json"
    );
    let token_salt = Salt::from([0u8; 32]);
    let token_compiled =
        Contract::load_sway_contract("../token_contract/out/debug/token_contract.bin", token_salt)
            .unwrap();

    // Get the contract ID and a handle to it
    let token_contract_id =
        Contract::deploy(&token_compiled, &provider, &wallet, TxParameters::default())
            .await
            .unwrap();
    let token_instance = TestToken::new(token_contract_id.to_string(), provider, wallet.clone());

    // Mint some tokens
    token_instance.mint_coins(77).call().await.unwrap();

    // Check the balance of the contract of its own asset
    let target = testtoken_mod::ContractId {
        value: token_contract_id.into(),
    };
    let asset_id = testtoken_mod::ContractId {
        value: token_contract_id.into(),
    };
    let result = token_instance
        .get_balance(target.clone(), asset_id.clone())
        .call()
        .await
        .unwrap();
    assert_eq!(result.value, 77);

    // mint some coins:
    let address = wallet.address();
    let address = testtoken_mod::Address {
        value: address.into(),
    };
    token_instance
        .transfer_coins_to_output(16, asset_id.clone(), address)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();

    let result = token_instance
        .get_balance(target.clone(), asset_id.clone())
        .call()
        .await
        .unwrap();
    assert_eq!(result.value, 61);

    /* let asset: [u8; 32] = token_contract_id.into();
    let call_params = CallParameters::new(Some(9), Some(AssetId::from(asset)));

    We should be able to deposit some tokens now
    let result = swayswap_instance
        .deposit()
        .call_params(call_params)
        .call()
        .await
        .unwrap();*/
}
