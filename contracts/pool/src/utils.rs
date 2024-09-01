use soroban_sdk::Env;

use crate::admin::get_admin;

pub fn require_admin(e: &Env) {
    let admin = get_admin(&e).signer;
    admin.require_auth();
}
