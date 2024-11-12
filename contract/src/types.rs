use soroban_sdk::{contracttype, Address};

#[derive(Clone, Debug)]
#[contracttype]
pub struct ContractConfig {
    pub owner: Address,
    pub soroswap_router: Address,
    pub usdc_token: Address,
    pub fee_recipient: Address,
    pub paused: bool,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct ExpectedOutputResult {
    pub amount_out: i128,
    pub min_amount_out: i128,
    pub slippage_rate: i128,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct SwapQuote {
    pub input_amount: i128,
    pub net_input_amount: i128,
    pub expected_output: i128,
    pub minimum_output: i128,
    pub fee_amount: i128,
    pub fee_rate: u32,
    pub slippage_rate: i128,
}
