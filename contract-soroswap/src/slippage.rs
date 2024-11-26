use crate::admin::read_admin;
use soroban_sdk::{symbol_short, Env, Symbol};

const SLIPPAGE: Symbol = symbol_short!("slippage");
const MIN_SLIPPAGE_BPS: u32 = 1;      // 0.01%
const MAX_SLIPPAGE_BPS: u32 = 5000;   // 50%
const DEFAULT_SLIPPAGE_BPS: u32 = 50;  // 0.5%

pub fn has_slippage(e: &Env) -> bool {
    if e.storage().instance().has(&SLIPPAGE) {
        true
    } else {
        false
    }
}

pub fn get_slippage(e: &Env) -> u32 {
    if has_slippage(&e) {
        e.storage().instance().get(&SLIPPAGE).unwrap()
    } else {
        DEFAULT_SLIPPAGE_BPS
    }
}

pub fn set_slippage(e: &Env, bps: u32) {
    let admin = read_admin(&e);
    admin.require_auth();

    if bps < MIN_SLIPPAGE_BPS || bps > MAX_SLIPPAGE_BPS {
        panic!("Invalid Slippage Value");
    }

    e.storage().instance().set(&SLIPPAGE, &bps);
}

pub fn calculate_min_amount(e: &Env, amount: i128) -> i128 {
    let bps = get_slippage(&e);

    amount - (amount * bps as i128 / 10_000)
}


