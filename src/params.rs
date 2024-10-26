//! Define the common parameters used in brands scheme.

use std::str::FromStr;

use diffie_hellman_groups::{MODPGroup, PrimeGroup};
use num_bigint::BigUint;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Common Parameters used in brands scheme.
#[derive(Clone, Serialize, Deserialize)]
pub struct Params {
    /// A customizable string being used in coin withdrawal and verification.
    pub(crate) scheme_key: String,

    /// p = prime
    pub(crate) p: BigUint,
    /// q = prime mod n, (i.e. max = n-1), p = 2q + 1, order(p)=n=2q
    pub(crate) q: BigUint,
    /// g^q mod p == 1, q != 2
    pub(crate) g: BigUint,
    /// g1^q mod p == 1, q != 2
    pub(crate) g1: BigUint,
    /// g2^q mod p == 1, q != 2
    pub(crate) g2: BigUint,
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

    /// Instantiates [Params] from a [MODPGroup] group which is a Diffie-Hellman group.
    /// The prime modulus `p` and Sophie Germain prime `q` are taken from the group.
    /// The distinct generators `g`, `g1`, and `g2` are generated randomly with bits
    /// ranging from 2 to the number of bits in the prime modulus `p`.
    ///
    ///
    /// ### Example
    /// ```
    /// use diffie_hellman_groups::{MODPGroup, MODPGroup5};
    ///
    /// let params = brands::Params::from_dh_group::<MODPGroup5>("brandskey".to_string());
    /// ```
    pub fn from_dh_group<G: MODPGroup>(scheme_key: String) -> Self {
        let p = G::prime_modulus();
        let q = G::sophie_garmain_prime();

        let mut rng = rand::thread_rng();
        let num_bits = rng.gen_range(2..p.bits() as usize);
        let g = PrimeGroup::new::<G>(num_bits).g;
        let g1;
        let g2;
        loop {
            let num_bits = rng.gen_range(2..p.bits() as usize);
            let g1_ = PrimeGroup::new::<G>(num_bits).g;

            let num_bits = rng.gen_range(2..p.bits() as usize);
            let g2_ = PrimeGroup::new::<G>(num_bits).g;

            if g != g1_ && g != g2_ && g1_ != g2_ {
                g1 = g1_;
                g2 = g2_;
                break;
            }
        }
        println!(
            "g bits: {}, g1 bits: {}, g2 bits: {} ",
            g.bits(),
            g1.bits(),
            g2.bits()
        );

        Self {
            scheme_key,
            p,
            q,
            g,
            g1,
            g2,
        }
    }
}
