use substrate_bn::{arith::U256, pairing, Fr, Group, G1, G2};

use crate::{
    crypto::bn::{
        hashes::{hash_g1_to_bytes, hash_to_g1, rand_fr},
        interpolation::interpolate,
        zkp_glow::{generate_zkp_glow, validate_zkp_glow, InstanceGlow, ZkpGlow},
    },
    types::dvrf::{Dvrf, DvrfConfig, DvrfPubParam, TrustedKeygenOutput},
};

#[derive(Clone)]
pub struct PartialEval {
    psig: G1,
    zkp_glow: ZkpGlow,
}

use super::{GlowDvrf, GlowDvrfHashless};

fn eval_at(x: u64, poly: &Vec<Fr>) -> Fr {
    let mut eval = Fr::zero();
    let x_as_fr = Fr::new(U256::from(x)).expect("only called by keygen and should not fail");
    // represents x ^ i starting with i = 0;
    let mut x_i = Fr::one();
    for &coeff_i in poly {
        eval = eval + (x_i * coeff_i);
        x_i = x_i * x_as_fr;
    }
    eval
}

impl Dvrf for GlowDvrfHashless {
    type SK = Fr;
    type VK = G1;
    type PK = G2;
    type Peval = PartialEval;
    type Out = G1;

    fn keygen(config: &DvrfConfig) -> TrustedKeygenOutput<Self> {
        // t random numbers => t-1 degree poly => t shares needed
        let coefficients: Vec<Fr> = (0..config.t).map(|_| rand_fr()).collect();
        let committee_sk = coefficients[0];
        let pk = G2::one() * committee_sk;

        let mut sks = Vec::new();
        let mut vks = Vec::new();

        for i in 1..config.n + 1 {
            let sk_i = eval_at(i as u64, &coefficients);
            let vk_i = G1::one() * sk_i;

            sks.push((i, sk_i));
            vks.push((i, vk_i));
        }

        let pp = DvrfPubParam {
            config: *config,
            vks,
            pk,
        };

        let out = TrustedKeygenOutput {
            pp,
            sks,
            #[cfg(test)]
            committee_sk: Some(committee_sk),
            #[cfg(not(test))]
            committee_sk: None,
        };

        out
    }

    fn part_eval(inp: &[u8], sk: &Self::SK, vk: &Self::VK) -> Self::Peval {
        let h = hash_to_g1(inp);
        let h_x = h * (*sk);
        let instance = &InstanceGlow {
            g: G1::one(),
            g_x: *vk,
            h,
            h_x,
        };
        PartialEval {
            psig: h_x,
            zkp_glow: generate_zkp_glow(instance, *sk),
        }
    }

    fn part_ver(inp: &[u8], vk: &Self::VK, peval: &Self::Peval) -> bool {
        let instance = &InstanceGlow {
            g: G1::one(),
            g_x: *vk,
            h: hash_to_g1(inp),
            h_x: peval.psig,
        };
        validate_zkp_glow(instance, &peval.zkp_glow)
    }

    fn aggregate(threshold: usize, pevals: Vec<(usize, Self::Peval)>) -> Result<Self::Out, String> {
        if pevals.len() < threshold {
            return Err(format!("insufficient partial evaluations"));
        }

        let mut peval_subset: Vec<(usize, G1)> = pevals
            .into_iter()
            .map(|(id, peval)| (id, peval.psig))
            .collect();
        peval_subset.truncate(threshold);
        Ok(interpolate(peval_subset))
    }

    fn out_ver_pk(inp: &[u8], pk: &Self::PK, out: &Self::Out) -> bool {
        pairing(*out, G2::one()) == pairing(hash_to_g1(inp), *pk)
    }
}

impl Dvrf for GlowDvrf {
    type SK = Fr;
    type VK = G1;
    type PK = G2;
    type Peval = PartialEval;
    type Out = ([u8; 32], G1);

    fn keygen(config: &DvrfConfig) -> TrustedKeygenOutput<Self> {
        let keys_hashless = GlowDvrfHashless::keygen(config);

        let pp = DvrfPubParam {
            pk: keys_hashless.pp.pk,
            config: keys_hashless.pp.config,
            vks: keys_hashless.pp.vks,
        };

        TrustedKeygenOutput {
            pp: pp,
            committee_sk: keys_hashless.committee_sk,
            sks: keys_hashless.sks,
        }
    }

    fn part_eval(inp: &[u8], sk: &Self::SK, vk: &Self::VK) -> Self::Peval {
        GlowDvrfHashless::part_eval(inp, sk, vk)
    }

    fn part_ver(inp: &[u8], vk: &Self::VK, peval: &Self::Peval) -> bool {
        GlowDvrfHashless::part_ver(inp, vk, peval)
    }

    fn aggregate(threshold: usize, pevals: Vec<(usize, Self::Peval)>) -> Result<Self::Out, String> {
        match GlowDvrfHashless::aggregate(threshold, pevals) {
            Ok(out) => Ok((hash_g1_to_bytes(&out), out)),
            Err(e) => Err(e),
        }
    }

    fn out_ver_pk(inp: &[u8], pk: &Self::PK, out: &Self::Out) -> bool {
        let (y, pi) = out;

        GlowDvrfHashless::out_ver_pk(inp, pk, pi) && hash_g1_to_bytes(pi).eq(y)
    }
}

#[cfg(test)]
mod test {
    use super::{GlowDvrf, GlowDvrfHashless};
    use crate::types::dvrf::test::{
        test_dvrf_aggregation_failure_insufficient_pevals,
        test_dvrf_aggregation_failure_invalid_pevals, test_dvrf_aggregation_success,
        test_dvrf_partial_evals,
    };

    #[test]
    fn test_dvrf_partial_evals_glow_hashless() {
        test_dvrf_partial_evals::<GlowDvrfHashless>();
    }
    #[test]
    fn test_dvrf_aggregation_success_glow_hashless() {
        test_dvrf_aggregation_success::<GlowDvrfHashless>();
    }
    #[test]
    fn test_dvrf_aggregation_failure_insufficient_pevals_glow_hashless() {
        test_dvrf_aggregation_failure_insufficient_pevals::<GlowDvrfHashless>();
    }
    #[test]
    fn test_dvrf_aggregation_failure_invalid_pevals_glow_hashless() {
        test_dvrf_aggregation_failure_invalid_pevals::<GlowDvrfHashless>();
    }

    #[test]
    fn test_dvrf_partial_evals_glow() {
        test_dvrf_partial_evals::<GlowDvrf>();
    }
    #[test]
    fn test_dvrf_aggregation_success_glow() {
        test_dvrf_aggregation_success::<GlowDvrf>();
    }
    #[test]
    fn test_dvrf_aggregation_failure_insufficient_pevals_glow() {
        test_dvrf_aggregation_failure_insufficient_pevals::<GlowDvrf>();
    }
    #[test]
    fn test_dvrf_aggregation_failure_invalid_pevals_glow() {
        test_dvrf_aggregation_failure_invalid_pevals::<GlowDvrf>();
    }
}
