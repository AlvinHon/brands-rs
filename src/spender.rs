use num_bigint::BigUint;

use crate::{
    coin::{Coin, PartialCoin, SpentCoin},
    params::{hash_to_number, random_number, Params},
    withdrawal::Withdrawal,
};

pub struct Spender {
    pub params: Params,
    pub i: BigUint,
    pub u1: BigUint,
    z: Option<BigUint>,
}

impl Spender {
    pub fn new(params: Params) -> Self {
        let u1 = random_number(&params.q);
        // i = g1^u1 mod p
        let i = params.g1.modpow(&u1, &params.p);
        Self {
            params,
            i,
            u1,
            z: None,
        }
    }

    pub fn set_registered(&mut self, z: BigUint) {
        self.z = Some(z);
    }

    pub fn withdraw(&self, a_by_issuer: &BigUint, b_by_issuer: &BigUint) -> Withdrawal {
        let partial_coin = PartialCoin {
            s: random_number(&self.params.q),
            x1: random_number(&self.params.q),
            x2: random_number(&self.params.q),
            u: random_number(&self.params.q),
            v: random_number(&self.params.q),
        };
        // A = (i * g2) ^ s
        let a = (&self.i * &self.params.g2).modpow(&partial_coin.s, &self.params.p);
        // B = g1^x2 * g2^x2
        let b = self.params.g1.modpow(&partial_coin.x1, &self.params.p)
            * self.params.g2.modpow(&partial_coin.x2, &self.params.p);
        // zd = z^s
        let zd = self
            .z
            .as_ref()
            .unwrap()
            .modpow(&partial_coin.s, &self.params.p);
        // ad = a^u * g^v
        let ad = a_by_issuer.modpow(&partial_coin.u, &self.params.p)
            * self.params.g.modpow(&partial_coin.v, &self.params.p);
        // bd = b^(s * u) * A^v
        let bd = b_by_issuer.modpow(&(&partial_coin.s * &partial_coin.u), &self.params.p)
            * a.modpow(&partial_coin.v, &self.params.p);
        // cd = Hash(A,B,zd,ad,bd)
        let challenge_d = hash_to_number(
            self.params.scheme_key.as_bytes(),
            &[
                a.to_bytes_le(),
                b.to_bytes_le(),
                zd.to_bytes_le(),
                ad.to_bytes_le(),
                bd.to_bytes_le(),
            ],
        ) % &self.params.p;
        // c = cd/u mod q
        let challenge =
            (&challenge_d * &partial_coin.u.modinv(&self.params.q).unwrap()) % &self.params.q;

        Withdrawal {
            a_by_issuer: a_by_issuer.clone(),
            b_by_issuer: b_by_issuer.clone(),
            challenge,
            challenge_d,
            a,
            b,
            zd,
            ad,
            bd,
            partial_coin,
        }
    }

    pub fn verify_withdrawal_response(
        &self,
        h: &BigUint,
        r: &BigUint,
        withdrawal: &Withdrawal,
    ) -> bool {
        // (i * g2)^r == z^c * b
        let lhs = (&self.i * &self.params.g2).modpow(r, &self.params.p);
        let rhs = (&self
            .z
            .as_ref()
            .unwrap()
            .modpow(&withdrawal.challenge, &self.params.p)
            * &withdrawal.b_by_issuer)
            % &self.params.p;
        if lhs != rhs {
            return false;
        }

        // g ^ r == h^c * a
        let lhs = self.params.g.modpow(r, &self.params.p);
        let rhs = (h.modpow(&withdrawal.challenge, &self.params.p) * &withdrawal.a_by_issuer)
            % &self.params.p;
        if lhs != rhs {
            return false;
        }

        true
    }

    pub fn make_coin(&self, response: BigUint, withdrawal: Withdrawal) -> Coin {
        let c1 = withdrawal.a;
        let c2 = withdrawal.b;
        let c3 = withdrawal.zd;
        let c4 = withdrawal.ad;
        let c5 = withdrawal.bd;
        // rd = ru + v mod q
        let c6 =
            (response * withdrawal.partial_coin.u + withdrawal.partial_coin.v) % &self.params.q;
        let cd = withdrawal.challenge_d;
        Coin {
            c1,
            c2,
            c3,
            c4,
            c5,
            c6,
            cd,
        }
    }

    pub fn spent_coin(&self, coin: &Coin, d: &BigUint, withdrawal: &Withdrawal) -> SpentCoin {
        // r1 = d(u1)s + x1 mod q
        let r1 = (d * &self.u1 * &withdrawal.partial_coin.s + &withdrawal.partial_coin.x1)
            % &self.params.q;
        // r2 = ds + x2 mod q
        let r2 = (d * &withdrawal.partial_coin.s + &withdrawal.partial_coin.x2) % &self.params.q;
        SpentCoin {
            c: coin.clone(),
            r1,
            r2,
        }
    }
}
