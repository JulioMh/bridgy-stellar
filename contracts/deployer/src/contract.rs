use crate::{
    admin::{get_admin, require_admin, set_admin},
    be::{read_be, write_be},
    storage_types::{
        read_all_pool, read_pool, read_tokens, write_pool, Authority, DataKey, PoolInfo,
    },
};

use soroban_sdk::{
    contract, contractimpl, symbol_short, vec,
    xdr::{FromXdr, ToXdr},
    Address, BytesN, Env, IntoVal, String, Val, Vec,
};

mod contract {
    soroban_sdk::contractimport!(file = "../../target/wasm32-unknown-unknown/release/pool.wasm");
}

#[contract]
pub struct BridgeDeployer;

#[contractimpl]
impl BridgeDeployer {
    pub fn init(e: Env, admin: Authority, be: BytesN<65>) {
        if e.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized")
        }

        e.storage().instance().set(&DataKey::Admin, &admin);
        e.storage().instance().set(&DataKey::Be, &be);
    }

    pub fn deploy(
        e: Env,
        deployer: Address,
        token: Address,
        other_chain_address: String,
        fee: u32,
        is_public: bool,
        split_fees: u32,
        owner: Authority,
        token_symbol: String,
    ) -> (Address, Val) {
        require_admin(&e, &deployer);
        let admin = get_admin(&e);
        let be = read_be(&e);
        let token_val: Val = token.clone().into_val(&e);
        let args: Vec<Val> = vec![
            &e,
            token_val,
            other_chain_address.into_val(&e),
            fee.into_val(&e),
            is_public.into_val(&e),
            split_fees.into_val(&e),
            owner.into_val(&e),
            admin.into_val(&e),
            be.into_val(&e),
            token_symbol.into_val(&e),
        ];
        let wasm_hash = e.deployer().upload_contract_wasm(contract::WASM);
        // // Only one pool per token
        let salt: BytesN<32> = BytesN::from_xdr(
            &e,
            &e.crypto().keccak256(&token.clone().to_xdr(&e)).to_xdr(&e),
        )
        .unwrap();
        // // Deploy the contract using the uploaded Wasm with given hash.
        let deployed_address = e.deployer().with_address(deployer, salt).deploy(wasm_hash);
        // Invoke the init function with the given arguments.
        let res: Val = e.invoke_contract(&deployed_address, &symbol_short!("init"), args);

        // Map token with his pool address
        write_pool(&e, token, deployed_address.clone(), token_symbol);

        // Return the contract ID of the deployed contract and the result of
        // invoking the init result.
        (deployed_address, res)
    }

    pub fn set_be(e: Env, user: Address, new_be: BytesN<65>) {
        write_be(&e, &user, &new_be)
    }

    pub fn get_be(e: Env) -> BytesN<65> {
        read_be(&e)
    }

    pub fn set_admin(e: Env, user: Address, new_admin: Authority) {
        set_admin(&e, &user, &new_admin)
    }

    pub fn get_admin(e: Env) -> Authority {
        get_admin(&e)
    }

    pub fn get_tokens(e: Env) -> Vec<Address> {
        read_tokens(&e)
    }

    pub fn get_pool(e: Env, token: Address) -> PoolInfo {
        read_pool(&e, token)
    }

    pub fn get_pools(e: Env) -> Vec<PoolInfo> {
        read_all_pool(&e)
    }
}
