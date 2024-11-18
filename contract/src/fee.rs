use crate::admin::read_admin;
use crate::types::{FeeInfo, StorageKey, FEE_DECIMALS};
use soroban_sdk::Env;

pub fn fee_check(e: &Env) -> bool {
    let key = StorageKey::FEE;

    if e.storage().instance().has(&key) {
        true
    } else {
        false
    }
}

pub fn fee_get(e: &Env) -> FeeInfo {
    let key = StorageKey::FEE;

    if !e.storage().instance().has(&key) {
        panic!("Fee wasn't initialized");
    }

    e.storage().instance().get(&key).unwrap()
}

pub fn fee_set(e: &Env, fee_info: &FeeInfo) {
    let admin = read_admin(&e);
    admin.require_auth();
    e.storage().instance().set(&StorageKey::FEE, fee_info);
}

pub fn calculate_fee(_e: &Env, fee_info: &FeeInfo, amount: u64) -> u64 {
    amount * (fee_info.fee_rate as u64) / (u64::pow(10, FEE_DECIMALS))
}
