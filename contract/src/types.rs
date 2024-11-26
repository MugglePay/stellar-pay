use soroban_sdk::{contracttype, Address};

pub(crate) const FEE_DECIMALS: u32 = 4;

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS; // 7 days
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS; // 6 days
pub(crate) const BALANCE_BUMP_AMOUNT: u32 = 1 * DAY_IN_LEDGERS; // 1 days
pub(crate) const SOROSWAP_FACTORY_ADDRESS: &str = "CD7OQTUYT7J4BN6EQWK6XX5SVDWFLKKFZ3PJ7GUJGLFIJSV52ZIQBWLG"; // Testnet
pub(crate) const SOROSWAP_ROUTER_ADDRESS: &str = "CCPEDSWPNWJTRIODNYO3THDJ6RYTMMAMDCGAIORRISXKGYU632Y475IF"; // Testnet

#[derive(Clone)]
#[contracttype]
pub struct FeeInfo {
    pub fee_rate: u32,
    pub fee_wallet: Address,
}

#[derive(Clone, Copy, PartialEq)]
#[contracttype]
pub enum OrderStatus {
    INIT = 0,
    ACTIVE = 1,
    COMPLETE = 2,
    CANCEL = 3
}

// Represents an offer managed by the Swap contract.
// If an sender wants to swap 1000 XLM for 20 USDC, the `send_amount` would be 1000
// and `recv_amount` would be 20
#[derive(Clone)]
#[contracttype]
pub struct OrderInfo {
    // Owner of this offer. Swaps send_token with recv_token.
    pub sender: Address,

    pub send_token: Address,
    pub recv_token: Address,

    // sender-defined amount of the input token
    pub send_amount: u64,
    // sender-defined amount of the recv token
    pub recv_amount: u64,
    pub min_output_amount: u64,

    pub status: OrderStatus
}

#[derive(Clone)]
#[contracttype]
pub struct OrderKey {
    pub sender: Address,
    pub send_token: Address,
    pub recv_token: Address,
    pub timestamp: u32,
}

#[derive(Clone)]
#[contracttype]
pub enum StorageKey {
    FEE,
    Allowance(Address),
    OrderCount,
    RegOrders(u32),
    ErrorCode,
    Admin,
}
