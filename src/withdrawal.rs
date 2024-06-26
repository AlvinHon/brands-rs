//! Defines structs that are useful in coin withdrawal process.

use num_bigint::BigUint;

use crate::coin::PartialCoin;

/// A mathematic representations of a state of a coin withdrawn from issuer. In this
/// state, the encapsulated information are not enough to form [Coin](crate::Coin) to
/// spend.
///
/// Continue along with the procotol flow, the function [ParitialCoin::from](crate::PartialCoin::from)
/// will be called in order to complete coin spending process.
///
/// When the coin has been spent, this withdrawal is no longer useful.
pub struct Withdrawal {
    /// a
    pub(crate) a_by_issuer: BigUint,
    /// b
    pub(crate) b_by_issuer: BigUint,
    pub(crate) challenge_d: BigUint,
    /// A
    pub(crate) a: BigUint,
    /// B
    pub(crate) b: BigUint,
    pub(crate) zd: BigUint,
    pub(crate) ad: BigUint,
    pub(crate) bd: BigUint,
    pub(crate) partial_coin: PartialCoin,
}

/// A challenge created by spender to issuer during coin withdrawal process, by
/// calling the method [withdraw](crate::Spender::withdraw).
///
/// This challenge will be used from issuer to create response, by calling the method
/// [withdrawal_response](crate::Issuer::withdrawal_response).
pub struct WithdrawalChallenge {
    pub(crate) c: BigUint,
}

/// A repsonse to the challenge created by issuer.
///
/// This will be used by the spender to make a coin, by calling the method
/// [make_coin](crate::Spender::make_coin).
///
/// The spender can also optionally verify the response beforehand, by calling
/// the method [verify_withdrawal_response](crate::Spender::verify_withdrawal_response).
pub struct WithdrawalResponse {
    pub(crate) r: BigUint,
}

/// Contains the parameters created by issuer. They are used by spender for
/// creation of a [Withdrawal] during the coin withdrawal process.
pub struct WithdrawalParams {
    pub(crate) a: BigUint,
    pub(crate) b: BigUint,
}

/// Contains the parameters created by issuer. They are used by issuer for
/// creation of a [WithdrawalResponse] during the coin withdrawal process.
pub struct WithdrawalResponseParams {
    pub(crate) w: BigUint,
}
