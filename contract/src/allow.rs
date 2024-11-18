use crate::admin::read_admin;
use crate::types::StorageKey;
use soroban_sdk::{log, Address, Env};

pub fn allow_set(e: &Env, token_addr: &Address) {
    let key = StorageKey::Allowance(token_addr.clone());
    let admin = read_admin(&e);
    admin.require_auth();
    if e.storage().instance().has(&key) && e.storage().instance().get::<_, bool>(&key).unwrap() {
        log!(&e, "current token already allowed");
        return;
    }

    e.storage().instance().set(&key, &true);
}

pub fn allow_reset(e: &Env, token_addr: &Address) {
    let key = StorageKey::Allowance(token_addr.clone());
    let admin = read_admin(&e);
    admin.require_auth();
    if !e.storage().instance().has(&key) || !e.storage().instance().get::<_, bool>(&key).unwrap() {
        log!(&e, "current token not allowed");
        return;
    }

    e.storage().instance().set(&key, &false);
}

pub fn allow_get(e: &Env, token: &Address) -> bool {
    let key = StorageKey::Allowance(token.clone());

    if e.storage().instance().has(&key) && e.storage().instance().get::<_, bool>(&key).unwrap() {
        true
    } else {
        false
    }
}
