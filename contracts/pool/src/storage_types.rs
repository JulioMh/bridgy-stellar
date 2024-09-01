use soroban_sdk::{contracterror, contracttype, Address, String};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;

pub(crate) const RELEASE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub(crate) const RELEASE_LIFETIME_THRESHOLD: u32 = RELEASE_BUMP_AMOUNT - DAY_IN_LEDGERS;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Pool,
    Admin,
    Be,
    Release(Address),
    Lock(Address),
}

#[derive(Clone)]
#[contracttype]
pub struct Pool {
    pub token: Address,
    pub other_chain_address: String,
    // 5 == 5%
    pub fee: u32,
    // 20 -> 20% admin  -  80% owner
    pub split_fees: u32,
    pub is_public: bool,
    pub token_symbol: String,
    pub owner: Authority,
}

#[derive(Clone)]
#[contracttype]
pub struct Release {
    pub last_claim: i128,
    pub total_claimed: i128,
}

#[derive(Clone)]
#[contracttype]
pub struct Lock {
    pub last_lock: i128,
    pub total_locked: i128,
}

#[derive(Clone)]
#[contracttype]
pub struct ReleaseLiqEvent {
    pub amount: i128,
    pub external_from: String,
    pub to: Address,
    pub token_other_chain: String,
    pub token: Address,
}

#[derive(Clone)]
#[contracttype]
pub struct Authority {
    pub signer: Address,
    pub fee_wallet: Address,
}

#[derive(Clone)]
#[contracttype]
pub struct ReleaseLiqPayload {
    pub amount: i128,
    pub to: Address,
    pub timestamp: i128,
    pub external_other_chain: String,
}

#[derive(Clone)]
#[contracttype]
pub struct LockLiqEvent {
    pub amount: i128,
    pub from: Address,
    pub to_other_chain: String,
    pub token_other_chain: String,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    Unauthorized = 1,
    InvalidCoupon = 2,
    PoolIsPrivate = 3,
    InvalidParams = 4,
    PoolInitialized = 5,
}
