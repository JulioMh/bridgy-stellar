use soroban_sdk::{contract, contractimpl, symbol_short, Address, BytesN, Env, String};

use crate::{
    admin::{get_admin, has_admin, set_admin},
    be::{read_be, write_be},
    coupons::{Coupon, CouponPayload},
    owner::{require_owner, set_owner},
    pool::{get_pool, set_pool},
    release::{read_release, write_release},
    storage_types::{Authority, LockLiqEvent, Pool, Release, ReleaseLiqEvent, ReleaseLiqPayload},
    token::{approve, transfer, transfer_in, transfer_out},
    utils::require_admin,
};

fn check_nonnegative_amount(amount: i128) {
    if amount < 0 {
        panic!("negative amount is not allowed: {}", amount)
    }
}

fn check_fee(fee: u32) {
    if fee > 100 {
        panic!("fee must be between 0 and 100 {}", fee)
    }
}

#[contract]
pub struct BridgePool;

#[contractimpl]
impl BridgePool {
    pub fn init(
        e: Env,
        token: Address,
        other_chain_address: String,
        fee: u32,
        is_public: bool,
        split_fees: u32,
        owner: Authority,
        admin: Authority,
        be: BytesN<65>,
        token_symbol: String,
    ) {
        if has_admin(&e) {
            panic!("already initialized")
        }

        check_fee(split_fees);
        check_fee(fee);

        let pool = Pool {
            token,
            other_chain_address,
            fee,
            split_fees,
            is_public,
            token_symbol,
            owner,
        };

        set_pool(&e, &pool);
        set_admin(&e, &admin);
        write_be(&e, &be);
    }

    pub fn release_liq(e: Env, coupon_payload: CouponPayload, payload: ReleaseLiqPayload) {
        let coupon = Coupon::new::<ReleaseLiqPayload>(&e, coupon_payload, &payload);
        coupon.verify(&e);

        let mut release = read_release(&e, payload.to.clone());
        assert!(
            payload.timestamp > release.last_claim,
            "coupon has already been used"
        );

        transfer_out(&e, &payload.to, &payload.amount);
        let pool = get_pool(&e);

        release.update(payload.timestamp, payload.amount);
        write_release(&e, payload.to.clone(), &release);

        e.events().publish(
            (symbol_short!("release"), payload.to.clone()),
            ReleaseLiqEvent {
                amount: payload.amount,
                external_from: payload.external_other_chain,
                token_other_chain: pool.other_chain_address,
                token: pool.token,
                to: payload.to,
            },
        );
    }

    pub fn lock_liq(e: Env, user: Address, amount: i128, to_other_chain: String) {
        user.require_auth();
        check_nonnegative_amount(amount);

        let pool = get_pool(&e);
        let owner = pool.owner;
        let admin = get_admin(&e);

        if !pool.is_public && user.ne(&owner.signer) {
            panic!("pool is private")
        }

        let as_fee = amount
            .checked_mul(pool.fee.into())
            .unwrap()
            .checked_div(100)
            .unwrap();
        let amount_to_admin = as_fee
            .checked_mul(pool.split_fees.into())
            .unwrap()
            .checked_div(100)
            .unwrap();
        let amount_to_owner = as_fee.checked_sub(amount_to_admin).unwrap();

        let to_receive = amount.checked_sub(as_fee).unwrap();

        transfer_in(&e, &user, &to_receive);
        transfer(&e, &user, &admin.fee_wallet, &amount_to_admin);
        transfer(&e, &user, &owner.fee_wallet, &amount_to_owner);

        e.events().publish(
            (symbol_short!("locked"),),
            LockLiqEvent {
                amount: to_receive,
                from: user,
                token_other_chain: pool.other_chain_address,
                to_other_chain,
            },
        );
    }

    pub fn approve(e: Env, from: Address, amount: i128) {
        from.require_auth();
        approve(&e, &from, &amount);
    }

    pub fn get_pool(e: Env) -> Pool {
        get_pool(&e)
    }

    pub fn get_release(e: Env, user: Address) -> Release {
        read_release(&e, user)
    }

    // ADMIN
    pub fn set_split(e: Env, split_fees: u32) {
        check_fee(split_fees);
        require_admin(&e);

        let mut pool = get_pool(&e);
        pool.split_fees = split_fees;
        set_pool(&e, &pool);
    }

    pub fn set_admin(e: Env, admin: Authority) {
        require_admin(&e);

        set_admin(&e, &admin);
    }

    pub fn set_be(e: Env, be: BytesN<65>) {
        require_admin(&e);

        write_be(&e, &be);
    }

    pub fn get_admin(e: Env) -> Authority {
        get_admin(&e)
    }

    // OWNER
    pub fn set_visibility(e: Env, is_public: bool) {
        require_owner(&e);

        let mut pool = get_pool(&e);
        pool.is_public = is_public;
        set_pool(&e, &pool);
    }

    pub fn set_owner(e: Env, owner: Authority) {
        require_owner(&e);

        set_owner(&e, owner);
    }

    pub fn set_other_chain_address(e: Env, other_chain_address: String) {
        require_owner(&e);

        let mut pool = get_pool(&e);
        pool.other_chain_address = other_chain_address;
        set_pool(&e, &pool);
    }

    pub fn set_fee(e: Env, fee: u32) {
        check_fee(fee);
        require_owner(&e);

        let mut pool = get_pool(&e);
        pool.fee = fee;
        set_pool(&e, &pool);
    }

    pub fn get_owner(e: Env) -> Authority {
        get_pool(&e).owner
    }

    pub fn get_be(e: Env) -> BytesN<65> {
        read_be(&e)
    }
}
