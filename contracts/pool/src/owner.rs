use soroban_sdk::Env;

use crate::{
    pool::{get_pool, set_pool},
    storage_types::Authority,
};

pub fn set_owner(e: &Env, owner: Authority) {
    let mut pool = get_pool(&e);
    pool.owner = owner;
    set_pool(&e, &pool);
}

pub fn require_owner(e: &Env) {
    let owner = get_pool(&e).owner.signer;
    owner.require_auth();
}
