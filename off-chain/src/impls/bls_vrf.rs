use super::BlsVrfHashless;
use crate::crypto::bn::hashes::{hash_to_g1, rand_fr};
use crate::types::vrf::Vrf;
use substrate_bn::{pairing, Fr, Group, G1, G2};

impl Vrf for BlsVrfHashless {
    type SK = Fr;
    type PK = G2;
    type Out = G1;

    fn keygen() -> (Self::SK, Self::PK) {
        let sk = rand_fr();
        let pk = G2::one() * sk;
        (sk, pk)
    }

    fn eval(inp: &[u8], sk: &Self::SK, _pk: &Self::PK) -> Self::Out {
        hash_to_g1(inp) * (*sk)
    }

    fn ver(inp: &[u8], pk: &Self::PK, out: &Self::Out) -> bool {
        pairing(*out, G2::one()) == pairing(hash_to_g1(inp), *pk)
    }
}

#[cfg(test)]
mod test {
    use super::BlsVrfHashless;
    use crate::types::vrf::test::{vrf_eval_ver, vrf_wrong_input_fails, vrf_wrong_pk_fails};

    #[test]
    fn bls_vrf_eval_ver() {
        vrf_eval_ver::<BlsVrfHashless>();
    }
    #[test]
    fn bls_vrf_wrong_input_fails() {
        vrf_wrong_input_fails::<BlsVrfHashless>();
    }
    #[test]
    fn bls_vrf_wrong_pk_fails() {
        vrf_wrong_pk_fails::<BlsVrfHashless>();
    }
}
