//! Implements of the protocol steps involved by an Issuer in the scheme.

use num_bigint::BigUint;

use crate::{
    cryptographics::random_number,
    params::Params,
    withdrawal::{WithdrawalChallenge, WithdrawalResponse},
    Identity, RegistrationID, WithdrawalParams, WithdrawalResponseParams,
};

/// A mathematic representation of a coin issuer in the scheme, which implements
/// methods for account registration, coins withdrawal and detecting double spent
/// coins.
pub struct Issuer {
    /// The public scheme parameters.
    pub params: Params,
    /// Identity of the issuer.
    pub h: Identity,
    /// The secret key of the issuer.
    ///
    /// (x, H) key pair by issuer, x is secret key
    x: BigUint,
}

impl Issuer {
    pub fn new(params: Params) -> Self {
        let x = random_number(&params.q);
        // H = g^x
        let h = params.g.modpow(&x, &params.p);
        Self { params, h, x }
    }

    /// Registers for opening an account to a spender, and gives back the
    /// registration ID to spender.
    ///
    /// ## Mis-representation Attack
    /// There is an attack to the scheme involving a bad user manipulating a false value of `i`
    /// who can later double spend without being caught.
    /// It is necessary for the issuer to ensture the authentication of the registration process.
    pub fn register(&self, i: &Identity) -> RegistrationID {
        // z = (I * g2)^x
        (i * &self.params.g2).modpow(&self.x, &self.params.p)
    }

    /// Setting up the parameters for starting the withdrawal process which issues one
    /// coin to the spender.
    ///
    /// The parameters will be used for creating [Withdrawal](crate::Withdrawal) by spender, and
    /// [WithdrawalResponse](crate::WithdrawalResponse) by issuer.
    pub fn setup_withdrawal_params(
        &self,
        i: &Identity,
    ) -> (WithdrawalParams, WithdrawalResponseParams) {
        let w = random_number(&self.params.q);
        // a = g^w
        let a = self.params.g.modpow(&w, &self.params.p);
        // b = (i * g2)^w
        let b = (i * &self.params.g2).modpow(&w, &self.params.p);
        (WithdrawalParams { a, b }, WithdrawalResponseParams { w })
    }

    /// Returns a response to the spender in withdrawal process. The response will then be used by
    /// spender to make a coin.
    pub fn withdrawal_response(
        &self,
        withdrawal: WithdrawalResponseParams,
        challenge: &WithdrawalChallenge,
    ) -> WithdrawalResponse {
        // r = w + c*x mod q
        let r = (&withdrawal.w + &challenge.c * &self.x) % &self.params.q;
        WithdrawalResponse { r }
    }
}
