use soroban_sdk::Env;

use crate::storage_types::{Authority, DataKey};

pub fn has_admin(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().instance().has(&key)
}

pub fn get_admin(e: &Env) -> Authority {
    e.storage().instance().get(&DataKey::Admin).unwrap()
}

pub fn set_admin(e: &Env, admin: &Authority) {
    e.storage().instance().set(&DataKey::Admin, admin);
}
