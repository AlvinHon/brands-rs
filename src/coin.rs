use num_bigint::BigUint;

use crate::params::{hash_to_number, Params, CHALLENGE_KEY};

#[derive(Clone)]
pub struct PartialCoin {
    pub s: BigUint,
    pub x1: BigUint,
    pub x2: BigUint,
    pub u: BigUint,
    pub v: BigUint,
}

#[derive(Clone)]
pub struct Coin {
    pub c1: BigUint,
    pub c2: BigUint,
    pub c3: BigUint,
    pub c4: BigUint,
    pub c5: BigUint,
    pub c6: BigUint,
    pub cd: BigUint,
}

impl Coin {
    pub fn verify(&self, h: &BigUint, params: &Params) -> bool {
        if self.c1 == BigUint::from(1u64) {
            return false;
        }

        let ver_cd = hash_to_number(
            CHALLENGE_KEY.as_bytes(),
            &[
                self.c1.to_bytes_le(),
                self.c2.to_bytes_le(),
                self.c3.to_bytes_le(),
                self.c4.to_bytes_le(),
                self.c5.to_bytes_le(),
            ],
        ) % &params.p;

        if self.cd != ver_cd {
            println!("Coin::verify cd != ver_cd");
            return false;
        }

        // c4 * h^cd = g^c6
        let lhs = (&self.c4 * h.modpow(&self.cd, &params.p)) % &params.p;
        let rhs = params.g.modpow(&self.c6, &params.p);
        if lhs != rhs {
            println!("Coin::verify fail (1)");
            return false;
        }

        // c5 * c3^cd = c1^c6
        let lhs = (&self.c5 * self.c3.modpow(&self.cd, &params.p)) % &params.p;
        let rhs = self.c1.modpow(&self.c6, &params.p);
        if lhs != rhs {
            println!("Coin::verify fail (2)");
            return false;
        }

        true
    }
}

pub struct SpentCoin {
    pub c: Coin,
    pub r1: BigUint,
    pub r2: BigUint,
}

impl SpentCoin {
    pub fn verify(&self, d: &BigUint, params: &Params) -> bool {
        // c1^d * c2 == g1^r1 * g2^r2
        let lhs = (self.c.c1.modpow(d, &params.p) * &self.c.c2) % &params.p;
        let rhs = (params.g1.modpow(&self.r1, &params.p) * params.g2.modpow(&self.r2, &params.p))
            % &params.p;
        lhs == rhs
    }
}
