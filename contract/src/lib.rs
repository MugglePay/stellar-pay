#![no_std]
use crate::types::{ContractConfig, ExpectedOutputResult, SwapQuote};
use errors::SwapError;
use soroban_sdk::{contract, contractimpl, symbol_short, token::Client as TokenClient, vec, Address, Env, String, Vec};
use crate::soroswap_router::{router_get_amounts_out, swap_exact_tokens_for_tokens};

mod errors;
mod fee;
mod slippage;
mod soroswap_router;
mod test;
mod types;

const SLIPPAGE_TOLERANCE: i128 = 5; // 0.5%
// const USDC_ADDRESS: &str = "CCKW6SMINDG6TUWJROIZ535EW2ZUJQEDGSKNIK3FBK26PAMBZDVK2BZA"; // Testnet
const USDC_ADDRESS: &str = "CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75"; // Mainnet
// const XLM_ADDRESS: &str = "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC"; // Testnet
const XLM_ADDRESS: &str = "CAS3J7GYLGXMF6TDJBBYYSE3HQ6BBSMLNUQ34T6TZMYMW2EVH34XOWMA"; // Mainnet
const SOROSWAP_ROUTER: &str = "CAG5LRYQ5JVEUI5TEID72EYOVX44TTUJT5BQR2J6J77FH65PCCFAJDDH";

#[contract]
pub struct XLMUSDCSwapContract;

#[contractimpl]
impl XLMUSDCSwapContract {
    /// Initialize the contract
    pub fn initialize(
        env: Env,
        owner: Address,
        soroswap_router: Address,
        usdc_token: Address,
        fee_recipient: Address,
    ) -> Result<(), SwapError> {
        // Prevent re-initialization
        if env.storage().instance().has(&symbol_short!("owner")) {
            return Err(SwapError::NotAuthorized);
        }

        // Store core configuration
        let config = ContractConfig {
            owner,
            soroswap_router: Self::get_soro_address(&env),
            usdc_token: Self::get_usdc_address(&env),
            fee_recipient,
            paused: false,
        };

        // Persist configuration
        env.storage()
            .instance()
            .set(&symbol_short!("config"), &config);

        // Configure initial fees and slippage
        fee::FeeManager::configure_fees(
            &env,
            &config.owner,
            25, // 0.25% default fee
            config.fee_recipient,
        )?;

        slippage::SlippageManager::set_slippage_tolerance(
            &env,
            &config.owner,
            SLIPPAGE_TOLERANCE, // 0.5% default slippage
        )?;

        Ok(())
    }

    pub fn get_usdc_address(env: &Env) -> Address {
        Address::from_string(&String::from_str(&env, USDC_ADDRESS))
    }

    pub fn get_xlm_address(env: &Env) -> Address {
        Address::from_string(&String::from_str(&env, XLM_ADDRESS))
    }

    pub fn get_soro_address(env: &Env) -> Address {
        Address::from_string(&String::from_str(&env, SOROSWAP_ROUTER))
    }

    /// Get current contract configuration
    pub fn get_config(env: Env) -> Result<ContractConfig, SwapError> {
        env.storage()
            .instance()
            .get(&symbol_short!("config"))
            .ok_or(SwapError::NotInitialized)
    }

    /// Update contract configuration
    pub fn update_config(
        env: Env,
        sender: Address,
        updates: ContractConfig,
    ) -> Result<(), SwapError> {
        // Verify sender is current owner
        let mut current_config = Self::get_config(env.clone())?;

        if sender != current_config.owner {
            return Err(SwapError::NotAuthorized);
        }

        // Update configuration fields
        // current_config.soroswap_router = updates.soroswap_router;
        // current_config.usdc_token = updates.usdc_token;
        current_config.fee_recipient = updates.fee_recipient;
        current_config.paused = updates.paused;

        // Persist updated configuration
        env.storage()
            .instance()
            .set(&symbol_short!("config"), &current_config);

        Ok(())
    }

    /// Main swap function
    pub fn swap(
        env: Env,
        buyer: Address,
        merchant: Address,
        amount_in: i128,
        min_amount_out: i128,
    ) -> Result<i128, SwapError> {
        // Retrieve configuration
        let config = Self::get_config(env.clone())?;

        // Check if contract is paused
        if config.paused {
            return Err(SwapError::ContractPaused);
        }

        // Validate inputs
        if amount_in <= 0 || min_amount_out <= 0 {
            return Err(SwapError::InsufficientAmount);
        }

        // Verify buyer authorization
        buyer.require_auth();

        // Native token address
        let native_address = Self::get_xlm_address(&env);

        // Calculate and transfer fees
        let (net_amount, fee_amount) =
            fee::FeeManager::calculate_and_transfer_fee(&env, amount_in, &config.fee_recipient)?;

        // Create router and token clients
        // let router_client = TokenClient::new(&env, &config.router);
        // Setting up Soroswap router client
        // let soroswap_router_client = SoroswapRouterClient::new(&env, &config.soroswap_router);

        let usdc_token_client = TokenClient::new(&env, &config.usdc_token);

        // Prepare swap path
        let path = Vec::from_array(&env, [native_address.clone(), config.usdc_token.clone()]);

        // Execute swap
        let swap_result = swap_exact_tokens_for_tokens(
            &env,
            &Self::get_soro_address(&env),
            net_amount,
            min_amount_out,
            &native_address.clone(),
            &config.usdc_token.clone(),
            &env.current_contract_address(),
            u64::MAX,
        );

        // Verify swap success
        // let usdc_amount = swap_result.get(1).expect("Invalid swap result");
        let usdc_amount = swap_result?;

        // Transfer USDC to merchant
        usdc_token_client.transfer(&env.current_contract_address(), &merchant, &usdc_amount);

        // Log swap event
        // env.events().publish(
        //     (symbol_short!("SWAP"), buyer.clone(), merchant.clone()),
        //     map![
        //         &env,
        //         symbol_short!("xlm") => amount_in,
        //         symbol_short!("net_input") => net_amount,
        //         symbol_short!("usdc_out") => usdc_amount,
        //         symbol_short!("fee") => fee_amount
        //     ]
        // );

        Ok(usdc_amount)
    }

    /// Get expected output with current configuration
    pub fn get_expected_output(
        env: Env,
        amount_in: i128,
    ) -> Result<ExpectedOutputResult, SwapError> {
        // Retrieve configuration
        let config = Self::get_config(env.clone())?;

        // println!("Config {:?}", config);

        // Validate input
        if amount_in <= 0 {
            return Err(SwapError::InsufficientAmount);
        }

        // Get current slippage tolerance
        // let slippage_rate = slippage::SlippageManager::get_slippage_tolerance(&env);

        // Native token address
        let native_address = Self::get_xlm_address(&env);

        // Get amounts out from router
        let results = router_get_amounts_out(
            &env,
            amount_in,
            &config.soroswap_router,
            &vec![&env, native_address.clone(), config.usdc_token.clone()]
        );

        // println!("AMOUNTS - {:?}", results);

        // Extract expected USDC amount
        let expected_amount = results.get(1).unwrap();

        // Calculate minimum amount with slippage
        let min_amount_out = (expected_amount * (1000 - SLIPPAGE_TOLERANCE)) / 1000;

        Ok(ExpectedOutputResult {
            amount_out: expected_amount,
            min_amount_out,
            slippage_rate: SLIPPAGE_TOLERANCE,
        })
    }

    /// Get comprehensive swap quote
    pub fn get_swap_quote(env: Env, amount_in: i128) -> Result<SwapQuote, SwapError> {
        // Get expected output
        let expected_output = Self::get_expected_output(env.clone(), amount_in)?;

        // Calculate fees
        let (net_input, fee_amount) = fee::FeeManager::calculate_fee(&env, amount_in);

        // Retrieve current fee rate
        let fee_rate = fee::FeeManager::get_current_fee_rate(&env);

        // Prepare detailed quote
        Ok(SwapQuote {
            input_amount: amount_in,
            net_input_amount: net_input,
            expected_output: expected_output.amount_out,
            minimum_output: expected_output.min_amount_out,
            fee_amount,
            fee_rate,
            slippage_rate: expected_output.slippage_rate,
        })
    }
}
