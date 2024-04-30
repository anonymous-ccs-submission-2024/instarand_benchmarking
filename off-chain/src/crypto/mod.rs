pub mod bn;
pub mod secp;

pub(crate) mod rng {
    use crypto::digest::Digest;
    use crypto::sha3::Sha3;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    pub fn seedable_rng(inp: &[u8]) -> StdRng {
        let mut h = [0u8; 32];

        let hasher = &mut Sha3::keccak256();
        hasher.input(inp);
        hasher.result(&mut h);

        SeedableRng::from_seed(h)
    }
}
