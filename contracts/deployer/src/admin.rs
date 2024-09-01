use soroban_sdk::{Address, Env};

use crate::storage_types::{Authority, DataKey};

pub fn get_admin(e: &Env) -> Authority {
    e.storage()
        .instance()
        .get(&DataKey::Admin)
        .unwrap_or_else(|| panic!("Admin not set"))
}

pub fn set_admin(e: &Env, user: &Address, new_admin: &Authority) {
    require_admin(e, user);
    e.storage().instance().set(&DataKey::Admin, new_admin);
}

pub fn require_admin(e: &Env, user: &Address) {
    user.require_auth();
    if user.ne(&get_admin(&e).signer) {
        panic!("admin required")
    }
}
