use soroban_sdk::{Address, Env, Symbol, symbol_short};
use crate::errors::SwapError;

pub struct SlippageManager;

impl SlippageManager {
    const DATA_KEY_SLIPPAGE: Symbol = symbol_short!("slippage");

    /// Set slippage tolerance
    pub fn set_slippage_tolerance(
        env: &Env,
        _sender: &Address,
        slippage_basis_points: i128
    ) -> Result<(), SwapError> {
        // Store slippage tolerance
        env.storage().instance().set(
            &Self::DATA_KEY_SLIPPAGE,
            &slippage_basis_points
        );

        Ok(())
    }

    /// Get current slippage tolerance
    pub fn get_slippage_tolerance(env: &Env) -> i128 {
        env.storage().instance()
            .get(&Self::DATA_KEY_SLIPPAGE)
            .unwrap_or(5)  // Default 0.5%
    }
}
