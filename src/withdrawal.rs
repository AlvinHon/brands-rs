use num_bigint::BigUint;

use crate::coin::PartialCoin;

#[derive(Clone)]
pub struct Withdrawal {
    /// a
    pub a_by_issuer: BigUint,
    /// b
    pub b_by_issuer: BigUint,
    pub challenge: BigUint,
    pub challenge_d: BigUint,
    /// A
    pub a: BigUint,
    /// B
    pub b: BigUint,
    pub zd: BigUint,
    pub ad: BigUint,
    pub bd: BigUint,
    pub partial_coin: PartialCoin,
}

pub struct WithdrawalParams {
    pub a: BigUint,
    pub b: BigUint,
    pub w: BigUint,
    // pub r: BigUint,
}
