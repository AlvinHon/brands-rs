//! Defines cryptograhic functions used for the library.

use hmac::{Hmac, Mac};
use num_bigint::{BigUint, RandBigInt};
use sha2::Sha256;

/// Returns a random number (mod m).
pub(crate) fn random_number(m: &BigUint) -> BigUint {
    // TODO : allow flexible random function
    let mut rng = rand::thread_rng();
    rng.gen_biguint_range(&BigUint::ZERO, m)
}

/// Converts a key-data pair into a number by using HMac-Sha256 over the content which is concatenation of
/// key and data.
pub(crate) fn hash_to_number<B: AsRef<[u8]>, T: AsRef<[B]>>(key: &[u8], data: &T) -> BigUint {
    // TODO : allow flexible hashing algorithm
    let strings_as_bytes: Vec<u8> = data
        .as_ref()
        .iter()
        .flat_map(|s| s.as_ref().to_vec())
        .collect();

    let hash_bytes = Hmac::<Sha256>::new_from_slice(key)
        .unwrap()
        .chain_update(strings_as_bytes)
        .finalize()
        .into_bytes()
        .to_vec();
    BigUint::from_bytes_le(&hash_bytes)
}
