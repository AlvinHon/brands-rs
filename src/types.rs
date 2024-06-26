//! Defines common types that are used for the crate;

use num_bigint::BigUint;

/// Identity of an actor in the scheme (i.e. a spender or a issuer).
pub type Identity = BigUint;

/// Registration Identifier provided by issuer to spender in registration process.
/// This is unique to the pair issuer-spender.
pub type RegistrationID = BigUint;
