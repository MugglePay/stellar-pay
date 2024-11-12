use soroban_sdk::{Address, Env, Symbol, symbol_short, token::Client as TokenClient, String};
use crate::errors::SwapError;

pub struct FeeManager;

impl FeeManager {
    const DATA_KEY_FEE_RATE: Symbol = symbol_short!("fee_rate");

    /// Configure fees (owner-only)
    pub fn configure_fees(
        env: &Env,
        _sender: &Address,
        fee_rate: u32,
        _fee_recipient: Address
    ) -> Result<(), SwapError> {
        // Validate fee rate
        if fee_rate > 1000 {  // Max 10%
            return Err(SwapError::InvalidFeeRate);
        }

        // Store fee rate
        env.storage().instance().set(
            &Self::DATA_KEY_FEE_RATE,
            &fee_rate
        );

        Ok(())
    }

    /// Calculate fee without transfer
    pub fn calculate_fee(
        env: &Env,
        amount_in: i128
    ) -> (i128, i128) {
        let fee_rate = Self::get_current_fee_rate(env);
        let fee_amount = (amount_in * fee_rate as i128) / 10_000;
        let net_amount = amount_in - fee_amount;

        (net_amount, fee_amount)
    }

    /// Calculate and transfer fee
    pub fn calculate_and_transfer_fee(
        env: &Env,
        amount_in: i128,
        fee_recipient: &Address
    ) -> Result<(i128, i128), SwapError> {
        // Calculate fee
        let (net_amount, fee_amount) = Self::calculate_fee(env, amount_in);

        // Native token address
        let native_address = Address::from_string(&String::from_str(&env, "native"));

        let native_contract = TokenClient::new(
            env,
            &native_address
        );

        // Transfer fee
        native_contract.transfer(
            &env.current_contract_address(),
            fee_recipient,
            &fee_amount
        );

        Ok((net_amount, fee_amount))
    }

    /// Get current fee rate
    pub fn get_current_fee_rate(env: &Env) -> u32 {
        env.storage().instance()
            .get(&Self::DATA_KEY_FEE_RATE)
            .unwrap_or(25)  // Default 0.25%
    }
}
