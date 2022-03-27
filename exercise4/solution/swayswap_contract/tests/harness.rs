use fuel_tx::{AssetId, Salt};
use fuels_abigen_macro::abigen;
use fuels_contract::{
    contract::Contract,
    parameters::{CallParameters, TxParameters},
};
use fuels_signers::util::test_helpers;

abigen!(TestSwayswap, "out/debug/swayswap_contract-abi.json");

#[tokio::test]
async fn swayswap() {
    let salt = Salt::from([0u8; 32]);
    let compiled = Contract::load_sway_contract("out/debug/swayswap_contract.bin", salt).unwrap();

    let (provider, wallet) = test_helpers::setup_test_provider_and_wallet().await;
    let id = Contract::deploy(&compiled, &provider, &wallet, TxParameters::default())
        .await
        .unwrap();

    let instance = TestSwayswap::new(id.to_string(), provider, wallet);

    /*let call_params = CallParameters::new(Some(11), None);
    let result = instance
        .deposit()
        .append_variable_outputs(1)
        .call_params(call_params)
        .call()
        .await
        .unwrap();*/
}
