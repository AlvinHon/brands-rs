use num_bigint::BigUint;

use crate::{
    coin::SpentCoin,
    params::{random_number, Params},
    withdrawal::WithdrawalParams,
};

pub struct Issuer {
    params: Params,
    pub h: BigUint,
    x: BigUint, // (x, H) key pair by issuer, x is secret key
}

impl Issuer {
    pub fn new(params: Params) -> Self {
        let x = random_number(&params.q);
        // H = g^x
        let h = params.g.modpow(&x, &params.p);
        Self { params, h, x }
    }

    pub fn register(&self, i: &BigUint) -> BigUint {
        // z = (I * g2)^x
        (i * &self.params.g2).modpow(&self.x, &self.params.p)
    }

    pub fn withdrawal_params(&self, i: &BigUint) -> WithdrawalParams {
        let w = random_number(&self.params.q);
        // a = g^w
        let a = self.params.g.modpow(&w, &self.params.p);
        // b = (i * g2)^w
        let b = (i * &self.params.g2).modpow(&w, &self.params.p);
        WithdrawalParams { w, a, b }
    }

    pub fn withdrawal_response(&self, withdrawal: &WithdrawalParams, c: &BigUint) -> BigUint {
        // r = w + c*x mod q
        (&withdrawal.w + c * &self.x) % &self.params.q
    }

    pub fn double_spender_identity(&self, c1: &SpentCoin, c2: &SpentCoin) -> BigUint {
        // g1 ^ ( (r1-r1') / (r2-r2') )
        let r1_diff = if c1.r1 > c2.r1 {
            &c1.r1 - &c2.r1
        } else {
            (&c1.r1 + &self.params.q - &c2.r1) % &self.params.q
        };
        let r2_diff = if c1.r2 > c2.r2 {
            &c1.r2 - &c2.r2
        } else {
            (&c1.r2 + &self.params.q - &c2.r2) % &self.params.q
        };
        let exponent = (r1_diff * r2_diff.modinv(&self.params.q).unwrap()) % &self.params.q;
        self.params.g1.modpow(&exponent, &self.params.p)
    }
}
