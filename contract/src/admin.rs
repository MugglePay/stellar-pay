use soroban_sdk::{
    token::{self, TokenClient},
    Address, Env,
};

use crate::types::StorageKey;

pub fn has_admin(e: &Env) -> bool {
    e.storage().instance().has(&StorageKey::Admin)
}

pub fn read_admin(e: &Env) -> Address {
    e.storage().instance().get(&StorageKey::Admin).unwrap()
}

pub fn write_admin(e: &Env, id: &Address) {
    e.storage().instance().set(&StorageKey::Admin, id);
}

pub fn get_token_client<'a>(env: &'a Env, token_address: &Address) -> TokenClient<'a> {
    token::Client::new(env, token_address)
}

pub fn check_token(env: &Env, token_address: &Address) {
    let token_client = get_token_client(env, &token_address);
    token_client.name();
}

pub fn get_token_balance(env: &Env, token_address: &Address, owner: &Address) -> i128 {
    let token_client = get_token_client(env, &token_address);
    token_client.balance(owner)
}