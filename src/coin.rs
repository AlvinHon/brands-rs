//! Defines structs related to a coin in various forms.
//!
//! There are many forms because a "coin" have different states during the whole life-cycle.
//! Those states include the state of a coin creation before coin withdrawal ([PartialCoin]),
//! before being spent ([Coin]), and after spent ([SpentCoin]).

use num_bigint::BigUint;

use crate::{cryptographics::hash_to_number, params::Params, Identity, Withdrawal};

/// A mathematic representation of a "coin" which has not yet complete its creation
/// during coin withdrawal process.
#[derive(Clone)]
pub struct PartialCoin {
    pub(crate) s: BigUint,
    pub(crate) x1: BigUint,
    pub(crate) x2: BigUint,
    pub(crate) u: BigUint,
    pub(crate) v: BigUint,
}

impl From<Withdrawal> for PartialCoin {
    /// Creates a partial coin from a Withdrawal, preparing for spending a coin.
    /// The withdrawal will be consumed and assume to be no longer useful (avoid
    /// double spend).
    fn from(w: Withdrawal) -> Self {
        w.partial_coin
    }
}

/// A mathematic representation of a "coin" which is ready to be spent.
#[derive(Clone, PartialEq, Eq)]
pub struct Coin {
    pub(crate) c1: BigUint,
    pub(crate) c2: BigUint,
    pub(crate) c3: BigUint,
    pub(crate) c4: BigUint,
    pub(crate) c5: BigUint,
    pub(crate) c6: BigUint,
    pub(crate) cd: BigUint,
}

/// A challenge created by coin receiver. The spender needs to give a response upon
/// receiving this chanllenge in order to prove the ownership of the coin.
pub struct CoinChallenge(pub(crate) BigUint);

impl CoinChallenge {
    pub fn new(message: &[u8], coin: &Coin) -> Self {
        Self(hash_to_number(
            message,
            &[coin.c1.to_bytes_le(), coin.c2.to_bytes_le()],
        ))
    }
}

impl Coin {
    /// Verifies if the coin is valid by using the issuer's identity (h) and the
    /// publicly known parameters. Returns true if the coin is valid.
    pub fn verify(&self, h: &Identity, params: &Params) -> bool {
        if self.c1 == BigUint::from(1u64) {
            return false;
        }

        let ver_cd = hash_to_number(
            params.scheme_key.as_bytes(),
            &[
                self.c1.to_bytes_le(),
                self.c2.to_bytes_le(),
                self.c3.to_bytes_le(),
                self.c4.to_bytes_le(),
                self.c5.to_bytes_le(),
            ],
        ) % &params.p;

        if self.cd != ver_cd {
            // println!("Coin::verify cd != ver_cd");
            return false;
        }

        // c4 * h^cd = g^c6
        let lhs = (&self.c4 * h.modpow(&self.cd, &params.p)) % &params.p;
        let rhs = params.g.modpow(&self.c6, &params.p);
        if lhs != rhs {
            // println!("Coin::verify fail (1)");
            return false;
        }

        // c5 * c3^cd = c1^c6
        let lhs = (&self.c5 * self.c3.modpow(&self.cd, &params.p)) % &params.p;
        let rhs = self.c1.modpow(&self.c6, &params.p);
        if lhs != rhs {
            // println!("Coin::verify fail (2)");
            return false;
        }

        true
    }
}

/// A mathematic representation of a "coin" which being spent. As compared to
/// the struct [Coin], it includes additional parameters which are created by
/// the spender upon a coin challenge during coin spending process.
pub struct SpentCoin {
    /// The coin sent by the spender.
    pub coin: Coin,
    pub(crate) r1: BigUint,
    pub(crate) r2: BigUint,
}

impl PartialEq for SpentCoin {
    /// Spent coins is said to be equivalent if they are having the same [Coin] information,
    /// regardless the additional parameters created by the spender during coin spending
    /// process. It is because a honous spender should not spend the same [Coin] to different
    /// receivers.
    fn eq(&self, other: &Self) -> bool {
        self.coin.eq(&other.coin)
    }
}
impl Eq for SpentCoin {}

impl SpentCoin {
    // Returns true if the spent coin is valid upon the coin challenge.
    pub fn verify(&self, challenge: &CoinChallenge, params: &Params) -> bool {
        // c1^d * c2 == g1^r1 * g2^r2
        let lhs = (self.coin.c1.modpow(&challenge.0, &params.p) * &self.coin.c2) % &params.p;
        let rhs = (params.g1.modpow(&self.r1, &params.p) * params.g2.modpow(&self.r2, &params.p))
            % &params.p;
        lhs == rhs
    }

    /// Given a double spent coin, compute the identity of the double spender.
    ///
    /// ## Panics
    /// Panics if the double_spent_coin is not referring to the same coin. It is caller responsibility
    /// to make sure the input `double_spent_coin` is actually "double spent".
    pub fn reveal_identity(&self, double_spent_coin: &SpentCoin, params: &Params) -> Identity {
        let coin_1 = self;
        let coin_2 = double_spent_coin;
        assert!(coin_1 == coin_2);

        // g1 ^ ( (r1-r1') / (r2-r2') )
        let r1_diff = if coin_1.r1 > coin_2.r1 {
            &coin_1.r1 - &coin_2.r1
        } else {
            (&coin_1.r1 + &params.q - &coin_2.r1) % &params.q
        };
        let r2_diff = if coin_1.r2 > coin_2.r2 {
            &coin_1.r2 - &coin_2.r2
        } else {
            (&coin_1.r2 + &params.q - &coin_2.r2) % &params.q
        };
        let exponent = (r1_diff * r2_diff.modinv(&params.q).unwrap()) % &params.q;
        params.g1.modpow(&exponent, &params.p)
    }
}
