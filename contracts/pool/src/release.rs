use soroban_sdk::{Address, Env};

use crate::storage_types::{DataKey, Release, RELEASE_BUMP_AMOUNT, RELEASE_LIFETIME_THRESHOLD};

pub fn read_release(e: &Env, addr: Address) -> Release {
    let key = DataKey::Release(addr);
    if let Some(release) = e.storage().persistent().get::<DataKey, Release>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, RELEASE_LIFETIME_THRESHOLD, RELEASE_BUMP_AMOUNT);
        release
    } else {
        Release {
            last_claim: 0,
            total_claimed: 0,
        }
    }
}

pub fn write_release(e: &Env, addr: Address, release: &Release) {
    let key = DataKey::Release(addr);
    e.storage().persistent().set(&key, release);
    e.storage()
        .persistent()
        .extend_ttl(&key, RELEASE_LIFETIME_THRESHOLD, RELEASE_BUMP_AMOUNT);
}

impl Release {
    pub fn update(&mut self, timestamp: i128, amount: i128) {
        self.last_claim = timestamp;
        self.total_claimed = self.total_claimed.checked_add(amount).unwrap();
    }
}
