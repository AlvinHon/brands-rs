//! Define the common parameters used in brands scheme.

use std::str::FromStr;

use hmac::{Hmac, Mac};
use num_bigint::BigUint;
use primes::{PrimeSet, Sieve};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

/// Common Parameters used in brands scheme.
#[derive(Clone, Serialize, Deserialize)]
pub struct Params {
    /// A customizable string being used in coin withdrawal and verification.
    pub scheme_key: String,

    /// p = prime
    pub p: BigUint,
    /// q = prime mod n, (i.e. max = n-1), p = 2q + 1, order(p)=n=2q
    pub q: BigUint,
    /// g^q mod p == 1, q != 2
    pub g: BigUint,
    /// g1^q mod p == 1, q != 2
    pub g1: BigUint,
    /// g2^q mod p == 1, q != 2
    pub g2: BigUint,
}

impl Params {
    /// Instantiates [Params] from string inputs. Returns None if the string inputs for those
    /// parametric values (e.g. "p", "q") cannot be converted into Unsigned big integers.
    ///
    /// This function does not verify whether the inputs are valid (i.e. satisfying the requirements
    /// of brands scheme).
    ///
    /// ### Example
    /// ```
    /// let params = brands::Params::from_str(
    ///     "brandskey".to_string(),
    ///     "170635838606142236835668582024526088839118584923917947104881361096573663241835425726334688227245750988284470206339098086628427330905070264154820140913414479495481939755079707182465802484020944276739164978360438985178968038653749024959908959885446602817557541340750337331201115159158715982367397805202392369959",
    ///     "85317919303071118417834291012263044419559292461958973552440680548286831620917712863167344113622875494142235103169549043314213665452535132077410070456707239747740969877539853591232901242010472138369582489180219492589484019326874512479954479942723301408778770670375168665600557579579357991183698902601196184979",
    ///     "78905550771707176472046196448658658754654071756606341285020444888851221712001014402581392171061103428557663126791572695604498371123013626618548119268438831780941305546724071040612015830836639524139258909464724634581470073606830394285772846821881118677913790493744652978276338707019197283548145299345563445342",
    ///     "144213202463066458950689095305115948799436864106778035179311009761777898846700415257265179855055640783875383274707858827879036088093691306491953244054442062637113833957623609837630797581860524549453053884680615629934658560796659252072641537163117203253862736053101508959059343335640009185013786003173143740486",
    ///     "103961858063657931242220807914123164620648299315033976046547900569904472805027212131284033634769267152657588195583605493290002050604375954536172541064476442340046198608255280588784539677337268545146088599238052090050779330669947961063002552055764161954608835115838286817546073467543570323501842149742495540876"
    /// ).unwrap();
    /// ```
    pub fn from_str(
        scheme_key: String,
        p: &str,
        q: &str,
        g: &str,
        g1: &str,
        g2: &str,
    ) -> Option<Self> {
        Some(Self {
            scheme_key,
            p: BigUint::from_str(p).ok()?,
            q: BigUint::from_str(q).ok()?,
            g: BigUint::from_str(g).ok()?,
            g1: BigUint::from_str(g1).ok()?,
            g2: BigUint::from_str(g2).ok()?,
        })
    }
}

/// Returns a [Params] with random parametric values.
/// This function generates parametric values which are in 64-bit integers. The
/// bit size is not very large hence this function is useful for development or
/// testing purpose only.
pub fn random_params(scheme_key: String, n: u64) -> Params {
    loop {
        let q = rand_prime(n);

        if q == 2 {
            // not accepting q=2
            continue;
        }

        let p = q * 2 + 1;
        if Sieve::new().is_prime(p) {
            let a = find_generator(p, q);
            let g1 = find_generator(p, q);
            let g2 = find_generator(p, q);
            return Params {
                scheme_key,
                p: BigUint::from(p),
                q: BigUint::from(q),
                g: BigUint::from(a),
                g1: BigUint::from(g1),
                g2: BigUint::from(g2),
            };
        }
    }
}

/// Generates random prime with value less than `max`. It is used in the function [random_params].
fn rand_prime(max: u64) -> u64 {
    let n = rand::random::<u64>() % max;
    let mut sieve = Sieve::new();
    sieve.iter().find(|x| x >= &n).unwrap()
}

/// Finds the generator x where x^q mod p == 1.It is used in the function [random_params].
fn find_generator(p: u64, q: u64) -> u64 {
    loop {
        let a = rand::random::<u64>() % p;
        let res = BigUint::from(a).modpow(&BigUint::from(q), &BigUint::from(p));
        if res == BigUint::from(1u64) {
            return a;
        }
    }
}

/// Returns a random number (mod m).
pub fn random_number(m: &BigUint) -> BigUint {
    BigUint::from(rand::random::<u64>()) % m
}

/// Converts a key-data pair into a number by using HMac-Sha256 over the content which is concatenation of
/// key and data.
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
