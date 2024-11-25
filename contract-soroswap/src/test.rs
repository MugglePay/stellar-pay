#![cfg(test)]
extern crate std;

use crate::contract::{MuggleDex, MuggleDexClient};

use soroban_sdk::{
    testutils::Address as _,
    Address, Env, String,
};

#[test]
fn test_full_flow() {
  let env = Env::default();
  env.mock_all_auths();

  let xlm_address =  "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC"; // XLM ADDRESS
  let usdc_address =  "CAAFIHB4I7WQMJMKC22CZVQNNX7EONWSOMT6SUXK6I3G3F6J4XFRWNDI"; // XLM ADDRESS
  let soroswap_router =  "CCPEDSWPNWJTRIODNYO3THDJ6RYTMMAMDCGAIORRISXKGYU632Y475IF"; // SOROSWAP ROUTER ADDRESS
  let soroswap_factory =  "CD7OQTUYT7J4BN6EQWK6XX5SVDWFLKKFZ3PJ7GUJGLFIJSV52ZIQBWLG"; // SOROSWAP FACTORY ADDRESS

  let input_token = Address::from_string(&String::from_str(&env, xlm_address));
  let output_token = Address::from_string(&String::from_str(&env, usdc_address));

  // Create test accounts
  let merchant = Address::generate(&env);
  let customer = Address::generate(&env);

  // Create mock router
  let router = Address::from_string(&String::from_str(&env, soroswap_router));
  let factory = Address::from_string(&String::from_str(&env, soroswap_factory));
  // let router = Address::generate(&env);
  // let factory = Address::generate(&env);

  // Deploy contract
  let contract_id = env.register_contract(None, MuggleDex);
  let contract_client = MuggleDexClient::new(&env, &contract_id);

  // Initialize contract test
  contract_client.initialize(&router, &factory);

  let config = contract_client.get_config();

  assert_eq!(config.router, router);
  assert_eq!(config.factory, factory);

  // Get expected amount
  let xlm_amount = 5000000; // 5 XLM
  let expected_amount = contract_client.get_expected_output(&xlm_amount, &input_token, &output_token);
  assert!(expected_amount > 0);


  // Test swap
  let amount_out = contract_client.swap(&customer, &merchant, &xlm_amount, &input_token, &output_token);
  assert_eq!(amount_out, expected_amount);
}
