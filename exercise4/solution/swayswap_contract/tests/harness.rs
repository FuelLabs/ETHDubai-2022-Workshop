use fuel_tx::Salt;
use fuels_abigen_macro::abigen;
use fuels_contract::{contract::Contract, parameters::TxParameters};
use fuels_signers::util::test_helpers;

abigen!(
    TestIncrementor,
    "out/debug/incrementor_contract-abi.json"
);

#[tokio::test]
async fn incrementor() {
    let salt = Salt::from([0u8; 32]);
    let compiled =
        Contract::load_sway_contract("out/debug/incrementor_contract.bin", salt)
            .unwrap();

    let (provider, wallet) = test_helpers::setup_test_provider_and_wallet().await;
    let id = Contract::deploy(&compiled, &provider, &wallet, TxParameters::default())
        .await
        .unwrap();

    let instance = TestIncrementor::new(id.to_string(), provider, wallet);

    let result = instance.get(5)
        .call()
        .await
        .unwrap();

    assert_eq!(result.value, 5);
}
