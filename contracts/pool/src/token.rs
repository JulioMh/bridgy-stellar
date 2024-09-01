use soroban_sdk::{token, Address, Env};

use crate::pool::get_pool;

pub fn approve(e: &Env, from: &Address, amount: &i128) {
    let pool = get_pool(e);
    let client = token::Client::new(e, &pool.token);
    client.approve(from, &e.current_contract_address(), amount, &500);
}

pub fn transfer(e: &Env, from: &Address, to: &Address, amount: &i128) {
    let pool = get_pool(e);
    let client = token::Client::new(e, &pool.token);
    client.transfer(from, to, amount);
}

pub fn transfer_in(e: &Env, from: &Address, amount: &i128) {
    transfer(e, &from, &e.current_contract_address(), amount)
}
pub fn transfer_out(e: &Env, to: &Address, amount: &i128) {
    transfer(e, &e.current_contract_address(), &to, amount)
}
