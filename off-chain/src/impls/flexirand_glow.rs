use substrate_bn::{AffineG1, Fr, G1, G2};

use crate::crypto::bn::hashes::hash_g1_to_bytes;
use crate::{
    crypto::bn::{
        hashes::{hash_to_g1, rand_fr},
        zkp_dl::{generate_zkp, validate_zkp, InstanceDl, ZkpDl},
    },
    types::{dvrf::Dvrf, flexirand::FlexiRand},
};

use super::{glow_dvrf::PartialEval, FlexiRandGlow, GlowDvrfHashless};

impl FlexiRand for FlexiRandGlow {
    type BlindingFactor = Fr;
    type BlindedInp = (G1, ZkpDl);
    type SK = Fr;
    type VK = G1;
    type PK = G2;
    type BlindedOut = G1;
    type UnblindedOut = ([u8; 32], G1);
    type Peval = PartialEval;

    type Dvrf = GlowDvrfHashless;

    fn keygen(
        config: &crate::types::dvrf::DvrfConfig,
    ) -> crate::types::dvrf::TrustedKeygenOutput<Self::Dvrf> {
        Self::Dvrf::keygen(config)
    }

    fn blinding_factor() -> Self::BlindingFactor {
        rand_fr()
    }

    fn blind_input_with_bf(inp: &[u8], bf: &Self::BlindingFactor) -> Self::BlindedInp {
        let h = hash_to_g1(inp);
        let instance = &InstanceDl { g: h, g_x: h * *bf };
        let zkp_dl = generate_zkp(instance, *bf);
        (h, zkp_dl)
    }

    fn inp_ver(inp: &[u8], blinded_input: &Self::BlindedInp) -> bool {
        let (blind_inp, proof) = blinded_input;
        let h = hash_to_g1(inp);
        let instance = &InstanceDl {
            g: h,
            g_x: *blind_inp,
        };
        validate_zkp(instance, proof)
    }

    fn part_eval(blinded_input: &Self::BlindedInp, sk: &Self::SK, vk: &Self::VK) -> Self::Peval {
        let (blinded_inp, _) = blinded_input;
        let blind_inp_bytes = g1_to_bytes(blinded_inp);
        Self::Dvrf::part_eval(&blind_inp_bytes, sk, vk)
    }
    fn part_ver(blinded_input: &Self::BlindedInp, vk: &Self::VK, peval: &Self::Peval) -> bool {
        let (blinded_inp, _) = blinded_input;
        let blind_inp_bytes = g1_to_bytes(blinded_inp);
        Self::Dvrf::part_ver(&blind_inp_bytes, vk, peval)
    }
    fn aggregate(
        threshold: usize,
        pevals: Vec<(usize, Self::Peval)>,
    ) -> Result<Self::BlindedOut, String> {
        Self::Dvrf::aggregate(threshold, pevals)
    }
    fn prever_with_pk(
        blinded_inp: &Self::BlindedInp,
        pk: &Self::PK,
        blinded_out: &Self::BlindedOut,
    ) -> bool {
        Self::Dvrf::out_ver_pk(&g1_to_bytes(&blinded_inp.0), pk, blinded_out)
    }
    fn unblind_output(bf: &Self::BlindingFactor, out: &Self::BlindedOut) -> Self::UnblindedOut {
        let bf_inv = bf.inverse().expect("should not fail on our benchmarks");
        Self::unblind_output_precomputed_inv(&bf_inv, out)
    }
    fn unblind_output_precomputed_inv(
        bf_inv: &Self::BlindingFactor,
        out: &Self::BlindedOut,
    ) -> Self::UnblindedOut {
        let pi = *out * *bf_inv;
        let y = hash_g1_to_bytes(&pi);
        (y, pi)
    }
    fn verify_out_with_pk(inp: &[u8], out: &Self::UnblindedOut, pk: &Self::PK) -> bool {
        let (y, pi) = out;
        Self::Dvrf::out_ver_pk(inp, pk, pi) && hash_g1_to_bytes(pi).eq(y)
    }
}

pub fn g1_to_bytes(p: &G1) -> [u8; 64] {
    let p_affine = AffineG1::from_jacobian(*p).expect("should not fail");
    let mut bytes = [0u8; 64];

    let _ = p_affine
        .x()
        .to_big_endian(&mut bytes[..32])
        .expect("serialization should not fail");
    let _ = p_affine
        .y()
        .to_big_endian(&mut bytes[32..])
        .expect("serialization should not fail");

    bytes
}
