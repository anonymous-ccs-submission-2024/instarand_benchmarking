use crypto::digest::Digest;

use crate::{crypto::rng::seedable_rng, impls::flexirand_glow::g1_to_bytes};

use crypto::sha3::Sha3;
use rand::rngs::OsRng;
use substrate_bn::{Fr, Group, G1};

pub fn rand_fr() -> Fr {
    Fr::random(&mut OsRng)
}

pub fn hash_to_fr(inp: &[u8]) -> Fr {
    Fr::random(&mut seedable_rng(inp))
}

pub fn hash_to_g1(inp: &[u8]) -> G1 {
    G1::random(&mut seedable_rng(inp))
}

pub fn hash_g1_to_bytes(inp: &G1) -> [u8; 32] {
    let inp_bytes = g1_to_bytes(inp);
    let mut h = [0u8; 32];

    let hasher = &mut Sha3::keccak256();
    hasher.input(&inp_bytes);
    hasher.result(&mut h);

    h
}

#[cfg(test)]
mod test {
    use super::{hash_to_fr, hash_to_g1, rand_fr};
    use crate::{crypto::bn::hashes::hash_g1_to_bytes, TEST_STRING_1, TEST_STRING_2};

    #[test]
    fn test_rand_fr() {
        let r1 = rand_fr();
        let r2 = rand_fr();

        assert_ne!(r1, r2);
    }
    #[test]
    fn test_hash_to_fr_same_input() {
        let inp = TEST_STRING_1.as_bytes();
        let out1 = hash_to_fr(inp);
        let out2 = hash_to_fr(inp);

        assert_eq!(out1, out2);
    }
    #[test]
    fn test_hash_to_fr_diff_inputs() {
        let inp_1 = TEST_STRING_1.as_bytes();
        let inp_2 = TEST_STRING_2.as_bytes();
        let out1 = hash_to_fr(inp_1);
        let out2 = hash_to_fr(inp_2);

        assert_ne!(out1, out2);
    }
    #[test]
    fn test_hash_to_g1_same_input() {
        let inp = TEST_STRING_1.as_bytes();
        let out1 = hash_to_g1(inp);
        let out2 = hash_to_g1(inp);

        assert_eq!(out1, out2);
    }
    #[test]
    fn test_hash_to_g1_diff_inputs() {
        let inp_1 = TEST_STRING_1.as_bytes();
        let inp_2 = TEST_STRING_2.as_bytes();
        let out1 = hash_to_g1(inp_1);
        let out2 = hash_to_g1(inp_2);

        assert_ne!(out1, out2);
    }
    #[test]
    fn test_hash_g1_to_bits_same_input() {
        let pre_inp = TEST_STRING_1.as_bytes();
        let inp = hash_to_g1(pre_inp);

        let out1 = hash_g1_to_bytes(&inp);
        let out2 = hash_g1_to_bytes(&inp);

        assert_eq!(out1, out2);
    }
    #[test]
    fn test_hash_g1_to_bits_diff_inputs() {
        let pre_inp_1 = TEST_STRING_1.as_bytes();
        let pre_inp_2 = TEST_STRING_2.as_bytes();
        let inp_1 = hash_to_g1(pre_inp_1);
        let inp_2 = hash_to_g1(pre_inp_2);

        let out1 = hash_g1_to_bytes(&inp_1);
        let out2 = hash_g1_to_bytes(&inp_2);

        assert_ne!(out1, out2);
    }
}
