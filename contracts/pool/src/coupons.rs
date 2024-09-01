use soroban_sdk::{contracttype, xdr::ToXdr, Bytes, BytesN, Env};

use crate::be::read_be;

#[derive(Clone)]
#[contracttype]
pub struct CouponPayload {
    pub recovery_id: u32,
    pub signature: BytesN<64>,
}

pub struct Coupon {
    pub recovery_id: u32,
    pub signature: BytesN<64>,
    pub payload: Bytes,
}

impl Coupon {
    pub fn new<T: Clone + ToXdr>(e: &Env, coupon: CouponPayload, payload: &T) -> Self {
        Coupon {
            recovery_id: coupon.recovery_id,
            signature: coupon.signature,
            payload: payload.clone().to_xdr(&e),
        }
    }

    pub fn verify(&self, env: &Env) {
        let hash = env.crypto().keccak256(&self.payload);
        let recovered_pubkey =
            env.crypto()
                .secp256k1_recover(&hash, &self.signature, self.recovery_id);

        if recovered_pubkey.ne(&read_be(&env)) {
            panic!("invalid coupon");
        }
    }
}
