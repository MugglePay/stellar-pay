use core::u32;

use crate::{
    error::Error,
    events::Events,
    types::{SwapConfig},
};
use soroban_sdk::{
    contract, 
    contractimpl, 
    token, 
    Address, 
    Env, 
    Symbol, 
    Vec, 
    String,
    symbol_short,
};

mod soroswap_router {
    soroban_sdk::contractimport!(file = "soroswap_router.wasm");
}
use soroswap_router::Client as SoroswapRouterClient;

const CONFIG_KEY: Symbol = symbol_short!("cfg");
const INITIALIZED_KEY: Symbol = symbol_short!("init");

#[contract]
pub struct MuggleDex;

#[contractimpl]
impl MuggleDex {
    pub fn initialize(
        env: Env,
        router: Address,
        factory: Address,
    ) -> Result<(), Error> {
        if Self::is_initialized(&env) {
            return Err(Error::AlreadyInitialized);
        }

        // Use the new SwapConfig::new method for validation
        let config = SwapConfig::new(router.clone(), factory.clone())
            .map_err(|_| Error::InvalidAddress)?;

        env.storage().instance().set(&CONFIG_KEY, &config);
        env.storage().instance().set(&INITIALIZED_KEY, &true);

        Events::contract_initialized(&env, &config.router);
        Ok(())
    }

    pub fn get_expected_output(
        env: Env,
        amount_in: i128,
        input_token: Address,
        output_token: Address
    ) -> Result<i128, Error> {
        let config = Self::get_config(&env)?;

        if amount_in <= 0 {
            return Err(Error::InvalidAmount);
        }

        let router_client = SoroswapRouterClient::new(&env, &config.router.clone());
        let path = Self::get_swap_path(&env,&input_token.clone(), &output_token.clone());

        let amounts = router_client.get_amounts_out(&config.factory.clone(), &amount_in, &path);

        Ok(amounts.get(amounts.len() - 1).unwrap())
    }

    pub fn swap(
        env: Env,
        customer: Address,
        merchant: Address,
        amount_in: i128,
        input_token: Address,
        output_token: Address,
    ) -> Result<i128, Error> {
        // Validate swap amount
        if amount_in <= 0 {
            return Err(Error::InvalidAmount);
        }

        customer.require_auth();

        // Validate configuration and swap parameters
        let config = Self::get_config(&env)?;

        let router_client = SoroswapRouterClient::new(&env, &config.router);
        let path = Self::get_swap_path(&env, &input_token.clone(), &output_token.clone());

        let expected_output = router_client.get_amounts_out(&config.factory.clone(), &amount_in, &path);
        let expected_amount_out = expected_output.get(expected_output.len() - 1).unwrap();

        // Transfer XLM from customer to contract
        let native_xlm = Address::from_string(&String::from_str(&env, "native"));
        let xlm_token = token::Client::new(&env, &native_xlm.clone());

        xlm_token.transfer(&customer, &env.current_contract_address(), &amount_in);
        xlm_token.approve(&env.current_contract_address(), &config.router, &amount_in, &u32::MAX);

        let amounts = router_client.swap_exact_tokens_for_tokens(
            &amount_in, 
            &expected_amount_out,
            &path, 
            &merchant, 
            &u64::MAX
        );
        let amount_out = amounts.get(amounts.len() - 1).unwrap();

        Events::swap_completed(&env, &customer, &merchant, amount_in, amount_out);
        Ok(amount_out)
    }

    // Helper functions
    fn is_initialized(env: &Env) -> bool {
        env.storage()
            .instance()
            .get::<Symbol, bool>(&INITIALIZED_KEY)
            .unwrap_or(false)
    }

    pub fn get_config(env: &Env) -> Result<SwapConfig, Error> {
        env.storage()
            .instance()
            .get::<Symbol, SwapConfig>(&CONFIG_KEY)
            .ok_or(Error::NotInitialized)
    }

    fn get_swap_path(env: &Env, input_token: &Address, output_token: &Address) -> Vec<Address> {
        let mut path = Vec::new(env);

        path.push_back(input_token.clone());
        path.push_back(output_token.clone());
        path
    }
}