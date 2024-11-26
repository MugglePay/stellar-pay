use core::u32;

use crate::{
    error::Error,
    events::Events,
    types::{SwapConfig},
    admin::{write_admin,read_admin, has_admin},
    slippage::{calculate_min_amount, set_slippage}
};
use soroban_sdk::{contract, contractimpl, token, Address, Env, Symbol, Vec, symbol_short, BytesN, vec, IntoVal};
use soroban_sdk::auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation};

mod soroswap_router {
    soroban_sdk::contractimport!(file = "soroswap_router.wasm");
}
use soroswap_router::Client as SoroswapRouterClient;

mod soroswap_pair {
    soroban_sdk::contractimport!(file = "soroswap_pair.wasm",);
}
use soroswap_pair::Client as SoroswapPairClient;

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
        admin: Address,
        slippage: Option<u32>,
    ) -> Result<(), Error> {
        if Self::is_initialized(&env) {
            return Err(Error::AlreadyInitialized);
        }

        // Use the new SwapConfig::new method for validation
        let config = SwapConfig::new(router.clone(), factory.clone())
            .map_err(|_| Error::InvalidAddress)?;

        env.storage().instance().set(&CONFIG_KEY, &config);
        env.storage().instance().set(&INITIALIZED_KEY, &true);

        write_admin(&env, &admin);
        
        if let Some(value) = slippage {
            set_slippage(&env, value);
        }

        Events::contract_initialized(&env);
        Ok(())
    }

    pub fn get_expected_amount(
        env: Env,
        amount_in: i128,
        input_token: Address,
        output_token: Address
    )-> Result<i128, Error> {
        let config = Self::get_config(&env)?;

        if amount_in <= 0 {
            return Err(Error::InvalidAmount);
        }

        let router_client = SoroswapRouterClient::new(&env, &config.router.clone());
        let path = Self::get_swap_path(&env, &input_token.clone(), &output_token.clone());

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
        expiration_ledger: u32,
    ) -> Result<i128, Error> {
        // Validate swap amount
        if amount_in <= 0 {
            return Err(Error::InvalidAmount);
        }

        let deadline = expiration_ledger;
        // Authorization for create order to verify their identity
        customer.require_auth();
        merchant.require_auth();

        let config = Self::get_config(&env)?;
        let path = Self::get_swap_path(&env, &input_token.clone(), &output_token.clone());
        let contract_address = env.current_contract_address();

        let send_token_client = token::Client::new(&env, &input_token.clone());

        // Check allowance between customer and contract address
        if send_token_client.allowance(&customer, &contract_address) < (amount_in as i128) {
            send_token_client.approve(
                &customer,
                &env.current_contract_address(),
                &amount_in,
                &deadline,
            );
        }

        // Transfer tokens from spender to contract
        send_token_client.transfer(
            &customer,
            &contract_address,
            &amount_in,
        );

        // Swap Flow
        let router_client = SoroswapRouterClient::new(&env, &config.router);
        let pair_contract_address = router_client.router_pair_for(&input_token.clone(), &output_token.clone());
        let pair_client = SoroswapPairClient::new(&env, &pair_contract_address);
        
        // Get the reserves of the tokens
        let (reserve_in, reserve_out) = pair_client.get_reserves();

        // Get Expected Amount
        let expected_out =  router_client.router_get_amount_out(&amount_in.clone(), &reserve_in, &reserve_out);
        let min_amount_out = calculate_min_amount(&env, expected_out);

        let output_token_client = token::Client::new(&env, &output_token.clone());

        // Authorize token transfer for the current contract
        // Without this tokens cannot be transferred from the current contract to the pair contract
        env.authorize_as_current_contract(vec![
            &env,
            InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: ContractContext {
                    contract: input_token.clone(),
                    fn_name: Symbol::new(&env, "transfer"),
                    args: (
                        env.current_contract_address(),
                        pair_contract_address,
                        amount_in,
                    )
                        .into_val(&env),
                },
                sub_invocations: vec![&env],
            }),
        ]);

        // Swap the tokens
        let swapped = router_client.swap_exact_tokens_for_tokens(
            &amount_in,
            &min_amount_out,
            &path,
            &env.current_contract_address(),
            &u64::MAX,
        );

        let total_swapped_amount = swapped.last().unwrap();

        // Transfer from contract address to merchant
        output_token_client.transfer(&env.current_contract_address(), &merchant.clone(), &total_swapped_amount);

        Events::swap_completed(&env, &customer, &merchant, amount_in, total_swapped_amount);
        Ok(total_swapped_amount)
    }


    // Get Config
    pub fn get_config(env: &Env) -> Result<SwapConfig, Error> {
        env.storage()
            .instance()
            .get::<Symbol, SwapConfig>(&CONFIG_KEY)
            .ok_or(Error::NotInitialized)
    }

    // Version
    pub fn version() -> u32 {
        2
    }

    // Upgrade
    pub fn upgrade(env: Env, new_wasm_hash: BytesN<32>) -> Result<(), Error> {
        if !has_admin(&env) {
            return Err(Error::NotInitialized);
        }

        let admin = read_admin(&env);
        admin.require_auth();

        env.deployer().update_current_contract_wasm(new_wasm_hash);

        Ok(())
    }

    // Set Slippage
    pub fn set_slippage(env: &Env, slippage: u32) -> Result<(), Error> {
        if !has_admin(&env) {
            return Err(Error::NotInitialized);
        }

        let admin = read_admin(&env);
        admin.require_auth();

        set_slippage(&env, slippage);

        Ok(())
    }

    // Helper functions
    fn is_initialized(env: &Env) -> bool {
        env.storage()
            .instance()
            .get::<Symbol, bool>(&INITIALIZED_KEY)
            .unwrap_or(false)
    }

    fn get_swap_path(env: &Env, input_token: &Address, output_token: &Address) -> Vec<Address> {
        let mut path = Vec::new(env);

        path.push_back(input_token.clone());
        path.push_back(output_token.clone());
        path
    }
}