//! Implements of the protocol steps involved by a Spender in the scheme.

use num_bigint::BigUint;

use crate::{
    coin::{Coin, CoinChallenge, PartialCoin, SpentCoin},
    cryptographics::{hash_to_number, random_number},
    params::Params,
    withdrawal::{Withdrawal, WithdrawalChallenge, WithdrawalResponse},
    Identity, RegistrationID, WithdrawalParams,
};

/// A mathematic representation of a spender in the scheme, which implements
/// methods for account registration, coins withdrawal and spending coins.
pub struct Spender {
    /// The public scheme parameters.
    pub params: Params,
    /// Identity of the spender.
    pub i: Identity,
    /// Secret value that the spender uses in spending a coin.
    u1: BigUint,
    /// The value given by the issuer for proving an issued coin.
    /// None if the spender has not yet complete registration with issuer.
    z: Option<RegistrationID>,
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

    /// Setting the value given by the issuer in registration process.
    pub fn set_registration_id(&mut self, registration_id: RegistrationID) {
        self.z = Some(registration_id);
    }

    /// Returns a Withdrawal by computations with the withdrawal parameters given by Issuer.
    /// A challenge is returned together for the spender to further check the validity of the
    /// issued coin.
    ///
    /// ## Panics
    /// Panics if the spender has not call [set_registration_id()](crate::Spender::set_registration_id)
    /// before (i.e. has not received an registration ID from issuer).
    pub fn withdraw(
        &self,
        withdrawal_spender_params: WithdrawalParams,
    ) -> (Withdrawal, WithdrawalChallenge) {
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
        let ad = withdrawal_spender_params
            .a
            .modpow(&partial_coin.u, &self.params.p)
            * self.params.g.modpow(&partial_coin.v, &self.params.p);
        // bd = b^(s * u) * A^v
        let bd = withdrawal_spender_params
            .b
            .modpow(&(&partial_coin.s * &partial_coin.u), &self.params.p)
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

        (
            Withdrawal {
                a_by_issuer: withdrawal_spender_params.a.clone(),
                b_by_issuer: withdrawal_spender_params.b.clone(),
                challenge_d,
                a,
                b,
                zd,
                ad,
                bd,
                partial_coin,
            },
            WithdrawalChallenge { c: challenge },
        )
    }

    /// Verifies the withdrawal response from issuer where the response will be used to create
    /// a coin. This verification is an optional step in the protocol.
    ///
    /// ## Panics
    /// Panics if the spender has not call [set_registration_id()](crate::Spender::set_registration_id)
    /// before (i.e. has not received an registration ID from issuer).
    pub fn verify_withdrawal_response(
        &self,
        h: &Identity,
        withdrawal: &Withdrawal,
        withdrawal_challenge: &WithdrawalChallenge,
        withdrawal_response: &WithdrawalResponse,
    ) -> bool {
        // (i * g2)^r == z^c * b
        let lhs = (&self.i * &self.params.g2).modpow(&withdrawal_response.r, &self.params.p);
        let rhs = (&self
            .z
            .as_ref()
            .unwrap()
            .modpow(&withdrawal_challenge.c, &self.params.p)
            * &withdrawal.b_by_issuer)
            % &self.params.p;
        if lhs != rhs {
            return false;
        }

        // g ^ r == h^c * a
        let lhs = self.params.g.modpow(&withdrawal_response.r, &self.params.p);
        let rhs = (h.modpow(&withdrawal_challenge.c, &self.params.p) * &withdrawal.a_by_issuer)
            % &self.params.p;
        if lhs != rhs {
            return false;
        }

        true
    }

    /// Makes a coin by the withdrawal response from issuer.
    pub fn make_coin(
        &self,
        withdrawal: &Withdrawal,
        withdrawal_response: WithdrawalResponse,
    ) -> Coin {
        let c1 = withdrawal.a.clone();
        let c2 = withdrawal.b.clone();
        let c3 = withdrawal.zd.clone();
        let c4 = withdrawal.ad.clone();
        let c5 = withdrawal.bd.clone();
        // rd = ru + v mod q
        let c6 = (withdrawal_response.r * &withdrawal.partial_coin.u + &withdrawal.partial_coin.v)
            % &self.params.q;
        let cd = withdrawal.challenge_d.clone();
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

    /// Spends the coin given challenge by verifier. This is supposed to
    /// be the last action by the spender on this coin.
    pub fn spend(
        &self,
        coin: Coin,
        partial_coin: PartialCoin,
        challenge: &CoinChallenge,
    ) -> SpentCoin {
        // r1 = d(u1)s + x1 mod q
        let r1 = (&challenge.0 * &self.u1 * &partial_coin.s + &partial_coin.x1) % &self.params.q;
        // r2 = ds + x2 mod q
        let r2 = (&challenge.0 * &partial_coin.s + &partial_coin.x2) % &self.params.q;
        SpentCoin { coin, r1, r2 }
    }
}
