// q = prime mod n, (i.e. max = n-1)
// p = prime,  p = 2q + 1, order(p)=n=2q
// a, g1, g2 s.t. x^q mod p == 1, q != 2

use hmac::{Hmac, Mac};
use num_bigint::BigUint;
use primes::{PrimeSet, Sieve};
use sha2::Sha256;

pub const CHALLENGE_KEY: &str = "brandskey";

#[derive(Clone)]
pub struct Params {
    pub p: BigUint,
    pub q: BigUint,
    pub g: BigUint,
    pub g1: BigUint,
    pub g2: BigUint,
}

impl Params {
    pub fn new(n: u64) -> Self {
        loop {
            let q = Self::rand_prime(n);

            if q == 2 {
                // not accepting q=2
                continue;
            }

            let p = q * 2 + 1;
            if Sieve::new().is_prime(p) {
                let a = Self::find_generator(p, q);
                let g1 = Self::find_generator(p, q);
                let g2 = Self::find_generator(p, q);
                return Self {
                    p: BigUint::from(p),
                    q: BigUint::from(q),
                    g: BigUint::from(a),
                    g1: BigUint::from(g1),
                    g2: BigUint::from(g2),
                };
            }
        }
    }

    fn rand_prime(max: u64) -> u64 {
        let n = rand::random::<u64>() % max;
        let mut sieve = Sieve::new();
        sieve.iter().find(|x| x >= &n).unwrap()
    }

    fn find_generator(p: u64, q: u64) -> u64 {
        loop {
            let a = rand::random::<u64>() % p;
            let res = BigUint::from(a).modpow(&BigUint::from(q), &BigUint::from(p));
            if res == BigUint::from(1u64) {
                return a;
            }
        }
    }
}

pub fn random_number(m: &BigUint) -> BigUint {
    BigUint::from(rand::random::<u64>()) % m
}

pub fn hash_to_number<B: AsRef<[u8]>, T: AsRef<[B]>>(key: &[u8], data: &T) -> BigUint {
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
