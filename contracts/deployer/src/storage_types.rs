use soroban_sdk::{contracttype, vec, Address, Env, String, Vec};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Be,
    Pools(Address),
    Tokens,
}

#[derive(Clone)]
#[contracttype]
pub struct Authority {
    pub signer: Address,
    pub fee_wallet: Address,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub struct PoolInfo {
    pub pool: Address,
    pub token_symbol: String,
    pub token_address: Address,
}

pub fn read_tokens(e: &Env) -> Vec<Address> {
    match e.storage().instance().get(&DataKey::Tokens) {
        Some(tokens) => tokens,
        _ => vec![e],
    }
}
fn write_token(e: &Env, token: Address) {
    let mut tokens = read_tokens(e);
    tokens.push_back(token);
    e.storage().instance().set(&DataKey::Tokens, &tokens);
}

pub fn read_pool(e: &Env, token: Address) -> PoolInfo {
    if let Some(pool) = e.storage().instance().get(&DataKey::Pools(token)) {
        pool
    } else {
        panic!("no pool for given token")
    }
}

pub fn read_all_pool(e: &Env) -> Vec<PoolInfo> {
    let tokens = read_tokens(e);
    let mut pools = Vec::<PoolInfo>::new(&e);
    for token in tokens.iter() {
        pools.push_back(read_pool(e, token));
    }
    pools
}

pub fn write_pool(e: &Env, token: Address, pool: Address, token_symbol: String) {
    write_token(e, token.clone());
    e.storage().instance().set(
        &DataKey::Pools(token.clone()),
        &PoolInfo {
            pool,
            token_symbol,
            token_address: token,
        },
    );
}
