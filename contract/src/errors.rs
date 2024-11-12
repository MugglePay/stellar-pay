use soroban_sdk::{Error, Env, contracterror, IntoVal, Val, TryFromVal, TryIntoVal};

// Update your SwapError to implement necessary traits
#[contracterror]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum SwapError {
    // Existing error variants
    NotInitialized = 1,
    NotAuthorized = 2,
    InsufficientAmount = 3,
    InvalidFeeRate = 4,
    SwapFailed = 5,
    TransferFailed = 6,
    ContractPaused = 7,
}

impl SwapError {
    // Convert SwapError to soroban_sdk::Error
    pub fn into_sdk_error(self) -> Error {
        Error::from_contract_error(self as u32)
    }

    // Convert soroban_sdk::Error to SwapError
    pub fn from_sdk_error(err: Error) -> Option<Self> {
        match err.get_code() {
            1 => Some(SwapError::NotAuthorized),
            2 => Some(SwapError::InsufficientAmount),
            3 => Some(SwapError::SwapFailed),
            4 => Some(SwapError::TransferFailed),
            5 => Some(SwapError::ContractPaused),
            _ => None
        }
    }
}

// Implement IntoVal for custom error type
// impl IntoVal<Env, Val> for SwapError {
//     fn into_val(self, env: &Env) -> Val {
//         (self as u32).into_val(env)
//     }
// }

// Implement TryFromVal for custom error type
// impl TryFromVal<Env, Val> for SwapError {
//     type Error = Error;
//
//     fn try_from_val(env: &Env, val: Val) -> Result<Self, Self::Error> {
//         let code: u32 = val.try_into_val(env)?;
//         Self::from_sdk_error(Error::from_contract_error(code))
//             .ok_or_else(|| Error::from_contract_error(0))
//     }
// }

// Error handling utility
pub struct ErrorUtil;

impl ErrorUtil {
    // Convert various error types
    pub fn convert_error<E: core::fmt::Debug>(err: E) -> SwapError {
        // Log the original error if needed
        // log::error!("Error occurred: {:?}", err);
        SwapError::SwapFailed
    }
}