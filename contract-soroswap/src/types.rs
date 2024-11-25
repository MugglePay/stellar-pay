use soroban_sdk::{
    contracttype, 
    Address
};

/// Represents the configuration for a swap
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwapConfig {
    /// The router address for executing swaps
    pub router: Address,
    
    /// The factory address for swap operations
    pub factory: Address,
}

/// Error types for swap-related operations
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SwapError {
    /// Contract not initialized
    NotInitialized,
    
    /// Swap amount is below minimum threshold
    AmountTooLow,
    
    /// Swap amount exceeds maximum limit
    AmountTooHigh,
    
    /// Insufficient output token balance
    InsufficientOutputBalance,
    
    /// Swap failed due to slippage
    SlippageTooHigh,
    
    /// Unauthorized swap attempt
    Unauthorized,
}

/// Utility functions for swap-related operations
impl SwapConfig {
    /// Creates a new swap configuration with validation
    pub fn new(
        router: Address, 
        factory: Address, 
    ) -> Result<Self, SwapError> {
        Ok(Self {
            router,
            factory,
        })
    }
}
