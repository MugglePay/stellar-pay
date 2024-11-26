use soroban_sdk::{symbol_short, Address, Env, Symbol};

const ADMIN: Symbol = symbol_short!("admin");

pub fn has_admin(e: &Env) -> bool {
    e.storage().instance().has(&ADMIN)
}

pub fn read_admin(e: &Env) -> Address {
    e.storage().instance().get(&ADMIN).unwrap()
}

pub fn write_admin(e: &Env, id: &Address) {
    e.storage().instance().set(&ADMIN, id);
}