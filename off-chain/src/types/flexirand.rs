use std::fmt::Debug;

use super::dvrf::{Dvrf, DvrfConfig, DvrfPubParam, TrustedKeygenOutput};

pub trait FlexiRand: Sized {
    type SK;
    type VK: Eq + Clone + Debug;
    type PK;
    type Peval: Clone;
    type BlindedOut;
    type BlindingFactor;
    type BlindedInp;
    type UnblindedOut;

    type Dvrf: Dvrf<
        SK = Self::SK,
        VK = Self::VK,
        PK = Self::PK,
        Peval = Self::Peval,
        Out = Self::BlindedOut,
    >;

    fn keygen(config: &DvrfConfig) -> TrustedKeygenOutput<Self::Dvrf>;

    fn blinding_factor() -> Self::BlindingFactor;

    fn blind_input_with_bf(inp: &[u8], bf: &Self::BlindingFactor) -> Self::BlindedInp;

    fn blind_input(inp: &[u8]) -> (Self::BlindingFactor, Self::BlindedInp) {
        let bf = Self::blinding_factor();
        let blinded_inp = Self::blind_input_with_bf(inp, &bf);
        (bf, blinded_inp)
    }

    fn inp_ver(inp: &[u8], blinded_input: &Self::BlindedInp) -> bool;

    fn part_eval(blinded_input: &Self::BlindedInp, sk: &Self::SK, vk: &Self::VK) -> Self::Peval;

    fn part_ver(blinded_input: &Self::BlindedInp, vk: &Self::VK, peval: &Self::Peval) -> bool;

    fn aggregate(
        threshold: usize,
        pevals: Vec<(usize, Self::Peval)>,
    ) -> Result<Self::BlindedOut, String>;

    fn aggregate_with_config(
        config: &DvrfConfig,
        pevals: Vec<(usize, Self::Peval)>,
    ) -> Result<Self::BlindedOut, String> {
        Self::aggregate(config.t, pevals)
    }

    fn aggregate_with_pp(
        pp: &DvrfPubParam<Self::Dvrf>,
        pevals: Vec<(usize, Self::Peval)>,
    ) -> Result<Self::BlindedOut, String> {
        Self::aggregate(pp.config.t, pevals)
    }

    fn prever_with_pk(
        blinded_inp: &Self::BlindedInp,
        pk: &Self::PK,
        blinded_out: &Self::BlindedOut,
    ) -> bool;
    fn prever_with_pp(
        blinded_inp: &Self::BlindedInp,
        pp: &DvrfPubParam<Self::Dvrf>,
        blinded_out: &Self::BlindedOut,
    ) -> bool {
        Self::prever_with_pk(blinded_inp, &pp.pk, blinded_out)
    }

    fn unblind_output(bf: &Self::BlindingFactor, out: &Self::BlindedOut) -> Self::UnblindedOut;
    fn unblind_output_precomputed_inv(
        bf_inv: &Self::BlindingFactor,
        out: &Self::BlindedOut,
    ) -> Self::UnblindedOut;

    fn verify_out_with_pk(inp: &[u8], out: &Self::UnblindedOut, pk: &Self::PK) -> bool;
    fn verify_out_with_pp(
        inp: &[u8],
        out: &Self::UnblindedOut,
        pp: &DvrfPubParam<Self::Dvrf>,
    ) -> bool {
        Self::verify_out_with_pk(inp, out, &pp.pk)
    }
}
