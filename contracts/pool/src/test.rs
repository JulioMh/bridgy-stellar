#![cfg(test)]

use secp256k1::rand::rngs::OsRng;
use secp256k1::{Message, Secp256k1};

use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, Events},
    token::{StellarAssetClient, TokenClient},
    xdr::ToXdr,
    Address, Env, FromVal, IntoVal, String,
};
use soroban_sdk::{Bytes, BytesN};

use crate::coupons::CouponPayload;
use crate::storage_types::ReleaseLiqEvent;
use crate::{
    contract::BridgePoolClient,
    storage_types::{Authority, LockLiqEvent, Pool, ReleaseLiqPayload},
    BridgePool,
};

fn create_token_contract<'a>(
    e: &Env,
    admin: &Address,
) -> (TokenClient<'a>, StellarAssetClient<'a>) {
    let contract_address = e.register_stellar_asset_contract(admin.clone());
    (
        TokenClient::new(e, &contract_address),
        StellarAssetClient::new(e, &contract_address),
    )
}

fn string_to_byte_n65(e: &Env, s: &str) -> Result<BytesN<65>, &'static str> {
    let vec = hex::decode(s).unwrap();
    let byte_array: [u8; 65] = vec
        .try_into()
        .map_err(|_| "Failed to convert to [u8; 65].")?;

    Ok(BytesN::from_array(&e, &byte_array))
}

fn sign(e: &Env, payload: &Bytes) -> (BytesN<65>, CouponPayload) {
    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);

    let hash = e.crypto().keccak256(payload);
    let message = Message::from_digest(hash.to_array());
    let (recovery_id, sig) = secp
        .sign_ecdsa_recoverable(&message, &secret_key)
        .serialize_compact();

    let v = recovery_id.to_i32();

    (
        BytesN::from_array(&e, &public_key.serialize_uncompressed()),
        CouponPayload {
            signature: BytesN::from_array(e, &sig),
            recovery_id: v.try_into().unwrap(),
        },
    )
}

fn init(
    e: &Env,
    is_public: bool,
) -> (
    BridgePoolClient,
    Pool,
    Authority,
    Authority,
    TokenClient,
    StellarAssetClient,
) {
    let client = BridgePoolClient::new(&e, &e.register_contract(None, BridgePool {}));

    let other_chain_address = String::from_str(&e, "3B5wuUrMEi5yATD7on46hKfej3pfmd7t1RKgrsN3pump");

    let admin = Authority {
        signer: Address::generate(&e),
        fee_wallet: Address::generate(&e),
    };

    let (token, token_admin) = create_token_contract(&e, &admin.signer);

    let owner = Authority {
        signer: Address::generate(&e),
        fee_wallet: Address::generate(&e),
    };

    let be =string_to_byte_n65(&e, "0452a51c1bef8056119d5f114af2d71a2e978a9b260e1a156c2af1a1643291b0e90c38da6d1bef18fc80588dddfc5344638954482c7de6613a0c5eef6ec2e36ee3").unwrap();
    let token_symbol = String::from_str(&e, "BILLY");

    client.init(
        &token.address,
        &other_chain_address,
        &15u32,
        &is_public,
        &30u32,
        &owner,
        &admin,
        &be,
        &token_symbol,
    );

    let pool = client.get_pool();

    (client, pool, admin, owner, token, token_admin)
}

#[test]
fn test_init() {
    let e = Env::default();
    let client = BridgePoolClient::new(&e, &e.register_contract(None, BridgePool {}));

    let other_chain_address = String::from_str(&e, "3B5wuUrMEi5yATD7on46hKfej3pfmd7t1RKgrsN3pump");

    let admin = Authority {
        signer: Address::generate(&e),
        fee_wallet: Address::generate(&e),
    };

    let (token, _) = create_token_contract(&e, &admin.signer);

    let owner = Authority {
        signer: Address::generate(&e),
        fee_wallet: Address::generate(&e),
    };

    let be =string_to_byte_n65(&e, "0452a51c1bef8056119d5f114af2d71a2e978a9b260e1a156c2af1a1643291b0e90c38da6d1bef18fc80588dddfc5344638954482c7de6613a0c5eef6ec2e36ee3").unwrap();
    let token_symbol = String::from_str(&e, "BILLY");

    client.init(
        &token.address,
        &other_chain_address,
        &15u32,
        &false,
        &30u32,
        &owner,
        &admin,
        &be,
        &token_symbol,
    );

    let pool = client.get_pool();
    let stored_admin = client.get_admin();
    let stored_owner = client.get_owner();
    let stored_be = client.get_be();

    assert_eq!(pool.fee, 15u32);
    assert_eq!(pool.split_fees, 30u32);
    assert_eq!(pool.other_chain_address, other_chain_address);
    assert_eq!(pool.token, token.address);
    assert_eq!(pool.is_public, false);
    assert_eq!(pool.token_symbol, token_symbol);

    assert_eq!(stored_owner.signer, owner.signer);
    assert_eq!(stored_owner.fee_wallet, owner.fee_wallet);
    assert_eq!(stored_admin.signer, admin.signer);
    assert_eq!(stored_admin.fee_wallet, admin.fee_wallet);
    assert_eq!(stored_be, be);
}

#[test]
#[should_panic(expected = "already initialized")]
fn already_initialized() {
    let e = Env::default();
    let client = BridgePoolClient::new(&e, &e.register_contract(None, BridgePool {}));

    let other_chain_address = String::from_str(&e, "3B5wuUrMEi5yATD7on46hKfej3pfmd7t1RKgrsN3pump");

    let admin = Authority {
        signer: Address::generate(&e),
        fee_wallet: Address::generate(&e),
    };

    let (token, _) = create_token_contract(&e, &admin.signer);

    let owner = Authority {
        signer: Address::generate(&e),
        fee_wallet: Address::generate(&e),
    };

    let be =string_to_byte_n65(&e, "0452a51c1bef8056119d5f114af2d71a2e978a9b260e1a156c2af1a1643291b0e90c38da6d1bef18fc80588dddfc5344638954482c7de6613a0c5eef6ec2e36ee3").unwrap();
    let token_symbol = String::from_str(&e, "BILLY");

    client.init(
        &token.address,
        &other_chain_address,
        &15u32,
        &false,
        &30u32,
        &owner,
        &admin,
        &be,
        &token_symbol,
    );
    client.init(
        &token.address,
        &other_chain_address,
        &15u32,
        &false,
        &30u32,
        &owner,
        &admin,
        &be,
        &token_symbol,
    );
}

#[test]
fn test_lock_liq() {
    let e = Env::default();
    e.mock_all_auths();
    let other_chain_wallet = String::from_str(&e, "4UEAHjxeHGetMjjPniMsks27j8CVWPZg2oSMDSAa7fHi");
    let user = Address::generate(&e);

    let (client, pool, admin, owner, token, token_admin) = init(&e, true);

    token_admin.mint(&user, &1500);
    assert_eq!(token.balance(&user), 1500);

    client.approve(&user, &1000);

    client.lock_liq(&user, &1000, &other_chain_wallet);

    assert_eq!(token.balance(&client.address), 850);
    assert_eq!(token.balance(&owner.fee_wallet), 105);
    assert_eq!(token.balance(&admin.fee_wallet), 45);

    let (contract_id, label, object) = e.events().all().last().unwrap();
    assert_eq!(contract_id, client.address);
    assert_eq!(label, (symbol_short!("locked"),).into_val(&e));

    let lock_event = LockLiqEvent::from_val(&e, &object);
    assert_eq!(lock_event.amount, 850);
    assert_eq!(lock_event.from, user);
    assert_eq!(lock_event.token_other_chain, pool.other_chain_address);
    assert_eq!(lock_event.to_other_chain, other_chain_wallet);
}

#[test]
#[should_panic(expected = "pool is private")]
fn pool_is_private() {
    let e = Env::default();
    e.mock_all_auths();

    let other_chain_wallet = String::from_str(&e, "4UEAHjxeHGetMjjPniMsks27j8CVWPZg2oSMDSAa7fHi");
    let user = Address::generate(&e);

    let (client, _, _, _, token, token_admin) = init(&e, false);

    token_admin.mint(&user, &1500);
    assert_eq!(token.balance(&user), 1500);

    client.approve(&user, &1000);

    client.lock_liq(&user, &1000, &other_chain_wallet);
}

#[test]
fn test_release_liq() {
    let e = Env::default();
    e.mock_all_auths();

    let other_chain_wallet = String::from_str(&e, "4UEAHjxeHGetMjjPniMsks27j8CVWPZg2oSMDSAa7fHi");
    let user = Address::generate(&e);

    let (client, pool, _, _, token, token_admin) = init(&e, true);

    token_admin.mint(&client.address, &1500);
    assert_eq!(token.balance(&client.address), 1500);

    let release_payload: ReleaseLiqPayload = ReleaseLiqPayload {
        amount: 1000,
        to: user.clone(),
        external_other_chain: other_chain_wallet.clone(),
        timestamp: 1,
    };

    let (public_key, coupon) = sign(&e, &release_payload.clone().to_xdr(&e));

    client.set_be(&public_key);

    client.release_liq(&coupon, &release_payload);

    let release = client.get_release(&user);

    assert_eq!(release.last_claim, 1);
    assert_eq!(release.total_claimed, 1000);

    assert_eq!(token.balance(&client.address), 500);
    assert_eq!(token.balance(&user), 1000);

    let (contract_id, label, object) = e.events().all().last().unwrap();
    assert_eq!(contract_id, client.address);
    assert_eq!(label, (symbol_short!("release"), user.clone()).into_val(&e));

    let release_event = ReleaseLiqEvent::from_val(&e, &object);
    assert_eq!(release_event.amount, 1000);
    assert_eq!(release_event.external_from, other_chain_wallet);
    assert_eq!(release_event.token_other_chain, pool.other_chain_address);
    assert_eq!(release_event.token, pool.token);
}

#[test]
#[should_panic(expected = "invalid coupon")]
fn bad_signer() {
    let e = Env::default();
    e.mock_all_auths();

    let other_chain_wallet = String::from_str(&e, "4UEAHjxeHGetMjjPniMsks27j8CVWPZg2oSMDSAa7fHi");
    let user = Address::generate(&e);

    let (client, _, _, _, token, token_admin) = init(&e, true);

    token_admin.mint(&client.address, &1500);
    assert_eq!(token.balance(&client.address), 1500);

    let release_payload: ReleaseLiqPayload = ReleaseLiqPayload {
        amount: 1000,
        to: user.clone(),
        external_other_chain: other_chain_wallet.clone(),
        timestamp: 1,
    };

    let (_, coupon) = sign(&e, &release_payload.clone().to_xdr(&e));

    client.release_liq(&coupon, &release_payload);
}

#[test]
#[should_panic(expected = "invalid coupon")]
fn bad_payload() {
    let e = Env::default();
    e.mock_all_auths();

    let other_chain_wallet = String::from_str(&e, "4UEAHjxeHGetMjjPniMsks27j8CVWPZg2oSMDSAa7fHi");
    let user = Address::generate(&e);

    let (client, _, _, _, token, token_admin) = init(&e, true);

    token_admin.mint(&client.address, &1500);
    assert_eq!(token.balance(&client.address), 1500);

    let mut release_payload: ReleaseLiqPayload = ReleaseLiqPayload {
        amount: 1000,
        to: user.clone(),
        external_other_chain: other_chain_wallet.clone(),
        timestamp: 1,
    };

    let (public_key, coupon) = sign(&e, &release_payload.clone().to_xdr(&e));

    client.set_be(&public_key);

    release_payload.amount = 2000;
    client.release_liq(&coupon, &release_payload);
}

#[test]
#[should_panic(expected = "coupon has already been used")]
fn coupon_is_invalid() {
    let e = Env::default();
    e.mock_all_auths();

    let other_chain_wallet = String::from_str(&e, "4UEAHjxeHGetMjjPniMsks27j8CVWPZg2oSMDSAa7fHi");
    let user = Address::generate(&e);

    let (client, _, _, _, token, token_admin) = init(&e, true);

    token_admin.mint(&client.address, &1500);
    assert_eq!(token.balance(&client.address), 1500);

    let release_payload: ReleaseLiqPayload = ReleaseLiqPayload {
        amount: 1000,
        to: user.clone(),
        external_other_chain: other_chain_wallet.clone(),
        timestamp: 1,
    };

    let (public_key, coupon) = sign(&e, &release_payload.clone().to_xdr(&e));

    client.set_be(&public_key);

    client.release_liq(&coupon, &release_payload);
    client.release_liq(&coupon, &release_payload);
}
