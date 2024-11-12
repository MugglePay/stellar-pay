#[cfg(test)]
mod test {
    use crate::{XLMUSDCSwapContract, XLMUSDCSwapContractClient};
    use soroban_sdk::{testutils::Address as _, Address, Env};
    use crate::types::ContractConfig;

    fn create_library_contract<'a>(e: &Env) -> XLMUSDCSwapContractClient<'a> {
        XLMUSDCSwapContractClient::new(e, &e.register_contract(None, XLMUSDCSwapContract {}))
    }

    struct XLMUSDCSwapContractTest<'a> {
        env: Env,
        contract: XLMUSDCSwapContractClient<'a>,
    }

    impl<'a> XLMUSDCSwapContractTest<'a> {
        fn setup() -> Self {
            let env = Env::default();
            env.mock_all_auths();

            let contract = create_library_contract(&env);
            XLMUSDCSwapContractTest {
                env,
                contract,
            }
        }
    }

    #[test]
    fn test_initialization() {
        let client = XLMUSDCSwapContractTest::setup();

        let owner = Address::generate(&client.env);
        let soroswap_router = client.contract.get_soro_address();
        let usdc_token = client.contract.get_usdc_address();
        let fee_recipient = Address::generate(&client.env);

        client.contract.initialize(
            &owner.clone(),
            &soroswap_router.clone(),
            &usdc_token.clone(),
            &fee_recipient.clone(),
        );

        // Verify that the configuration is stored properly
        let config = client.contract.get_config();
        assert_eq!(config.owner, owner);
        assert_eq!(config.soroswap_router, soroswap_router);
        assert_eq!(config.usdc_token, usdc_token);
        assert_eq!(config.fee_recipient, fee_recipient);
        assert!(!config.paused);
    }

    #[test]
    fn test_update_config() {
        let client = XLMUSDCSwapContractTest::setup();

        let owner = Address::generate(&client.env);
        let soroswap_router = client.contract.get_soro_address();
        let usdc_token = client.contract.get_usdc_address();
        let fee_recipient = Address::generate(&client.env);

        client.contract.initialize(
            &owner.clone(),
            &soroswap_router.clone(),
            &usdc_token.clone(),
            &fee_recipient.clone(),
        );

        let payload = ContractConfig {
            owner: owner.clone(),
            soroswap_router: soroswap_router.clone(),
            usdc_token: usdc_token.clone(),
            fee_recipient: Address::generate(&client.env),
            paused: true,
        };

        client.contract.update_config(&owner.clone(), &payload);

        let config = client.contract.get_config();
        assert_eq!(config.fee_recipient, payload.fee_recipient);
        assert!(config.paused);
    }

    // #[test]
    // fn test_expected_output() {
    //     let e = Env::default();
    //     let client = create_library_contract(&e);
    //
    //     let owner = Address::generate(&e);
    //     let soroswap_router = client.get_soro_address();
    //     let usdc_token = client.get_usdc_address();
    //
    //     let buyer_address = Address::generate(&e);
    //     let merchant_address = Address::generate(&e);
    //     let fee_recipient = Address::generate(&e);
    //
    //     client.initialize(
    //         &owner.clone(),
    //         &soroswap_router.clone(),
    //         &usdc_token.clone(),
    //         &fee_recipient.clone(),
    //     );
    //
    //     let xlm_amount = 10_0000000; // 10 XLM
    //     let result = client.get_expected_output(&xlm_amount);
    //
    //     println!("Otoke - {:?}", result);
    // }
}
