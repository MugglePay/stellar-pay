use soroban_sdk::{Address, Env};

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
