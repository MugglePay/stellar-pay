#![cfg(test)]
extern crate std;

use crate::contract::{MuggleDex, MuggleDexClient};

use soroban_sdk::{
    testutils::Address as _,
    Address, Env, String,
};
use soroban_sdk::token::{StellarAssetClient, TokenClient};

fn create_token_contract<'a>(
  e: &Env,
  admin: &Address,
) -> (Address, TokenClient<'a>, StellarAssetClient<'a>) {
  let contract_address = e
      .register_stellar_asset_contract_v2(admin.clone())
      .address();
  (
    contract_address.clone(),
    TokenClient::new(e, &contract_address),
    StellarAssetClient::new(e, &contract_address),
  )
}

#[test]
fn test_full_flow() {
  let env = Env::default();
  env.mock_all_auths();

  const MUL_VAL: u64 = u64::pow(10, 4);

  let xlm_address =  "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC"; // XLM ADDRESS
  let usdc_address =  "CAAFIHB4I7WQMJMKC22CZVQNNX7EONWSOMT6SUXK6I3G3F6J4XFRWNDI"; // XLM ADDRESS
  let soroswap_router =  "CCPEDSWPNWJTRIODNYO3THDJ6RYTMMAMDCGAIORRISXKGYU632Y475IF"; // SOROSWAP ROUTER ADDRESS
  let soroswap_factory =  "CD7OQTUYT7J4BN6EQWK6XX5SVDWFLKKFZ3PJ7GUJGLFIJSV52ZIQBWLG"; // SOROSWAP FACTORY ADDRESS

  let input_token = Address::from_string(&String::from_str(&env, xlm_address));
  let output_token = Address::from_string(&String::from_str(&env, usdc_address));

  // Create test accounts
  let admin = Address::generate(&env);
  let customer = Address::generate(&env);
  let merchant = Address::generate(&env);
  let deadline = 1391440;

  // Create mock router
  let router = Address::from_string(&String::from_str(&env, soroswap_router));
  let factory = Address::from_string(&String::from_str(&env, soroswap_factory));
  // let router = Address::generate(&env);
  // let factory = Address::generate(&env);

  // Input Token
  let (send_token_id, send_token_client, send_token_admin_client) =
      create_token_contract(&env, &admin);
  send_token_admin_client.mint(&customer.clone(), &(1000_i128 * MUL_VAL as i128));

  // Output Token
  let (recv_token_id, recv_token_client, recv_token_admin_client) =
      create_token_contract(&env, &admin);
  recv_token_admin_client.mint(&merchant.clone(), &(100_i128 * MUL_VAL as i128));

  // Deploy contract
  let contract_id = env.register_contract(None, MuggleDex);
  let contract_client = MuggleDexClient::new(&env, &contract_id);

  // Initialize contract test
  contract_client.initialize(&router, &factory, &admin, &None);

  let config = contract_client.get_config();

  assert_eq!(config.router, router);
  assert_eq!(config.factory, factory);

  // Get expected amount
  let xlm_amount = 5000000; // 5 XLM
  let expected_amount = contract_client.get_expected_amount(&xlm_amount, &input_token, &output_token);
  assert!(expected_amount > 0);


  // Test swap
  let amount_out = contract_client.swap(&customer, &merchant, &xlm_amount, &input_token, &output_token, &deadline);
  assert_eq!(amount_out, expected_amount);
}
