use fuel_tx::Salt;
use fuels_abigen_macro::abigen;
use fuels_contract::{contract::Contract, parameters::TxParameters};
use fuels_signers::util::test_helpers;

abigen!(
    IncTest,
    "out/debug/incrementor_contract-abi.json"
);

#[tokio::test]
async fn mint() {
    let salt = Salt::from([0u8; 32]);
    let compiled =
        Contract::load_sway_contract("out/debug/incrementor_contract.bin", salt)
            .unwrap();

    let (provider, wallet) = test_helpers::setup_test_provider_and_wallet().await;
    let id = Contract::deploy(&compiled, &provider, &wallet, TxParameters::default())
        .await
        .unwrap();

    let instance = IncTest::new(id.to_string(), provider, wallet);

    // Currently not working due to SDK + forc issues
    instance.initialize(1)
            .call()
            .await
            .unwrap();

    instance.increment(2)
            .call()
            .await
            .unwrap();

    instance.increment(5)
            .call()
            .await
            .unwrap();

    let result = instance.get()
            .call()
            .await
            .unwrap();

    asserteq!(result.value == 8)
}
