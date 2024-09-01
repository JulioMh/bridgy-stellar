use soroban_sdk::{Address, BytesN, Env};

use crate::{admin::require_admin, storage_types::DataKey};

pub fn read_be(e: &Env) -> BytesN<65> {
    e.storage().instance().get(&DataKey::Be).unwrap()
}

pub fn write_be(e: &Env, user: &Address, new_be: &BytesN<65>) {
    require_admin(e, user);
    e.storage().instance().set(&DataKey::Be, new_be);
}
