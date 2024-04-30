use std::fmt::Debug;

use crate::{DEFAULT_NUM_NODES, DEFAULT_THRESHOLD};

pub const DVRF_MAX_PARTIES: usize = 256;

#[derive(Clone, Copy)]
pub struct DvrfConfig {
    pub t: usize,
    pub n: usize,
}

impl DvrfConfig {
    pub fn validate(&self) -> bool {
        self.t != 0 && self.t <= self.n && self.n <= DVRF_MAX_PARTIES
    }
}

// #[cfg(test)]
impl Default for DvrfConfig {
    fn default() -> Self {
        DvrfConfig {
            t: DEFAULT_THRESHOLD,
            n: DEFAULT_NUM_NODES,
        }
    }
}

pub struct DvrfPubParam<T: Dvrf> {
    pub config: DvrfConfig,
    pub vks: Vec<(usize, T::VK)>,
    pub pk: T::PK,
}

impl<T: Dvrf> DvrfPubParam<T> {
    pub fn threshold(&self) -> usize {
        self.config.t
    }
    pub fn committee_size(&self) -> usize {
        self.config.n
    }
}

impl<T: Dvrf> DvrfPubParam<T> {
    pub fn vk_from_id(&self, id: usize) -> Result<Option<T::VK>, String> {
        let matches: Vec<_> = self.vks.iter().filter(|(i, _)| *i == id).collect();

        match matches.len() {
            0 => Ok(None),
            1 => {
                let (_, vk) = matches[0].clone();
                Ok(Some(vk))
            }
            _ => Err(format!("multiple vks for id {}", id)),
        }
    }

    pub fn contains_id(&self, id: usize) -> Result<bool, String> {
        let (indices, _): (Vec<usize>, Vec<_>) = self
            .vks
            .iter()
            .filter(|(i, _vk_i)| *i == id)
            .map(|(i, vk)| (i, vk))
            .unzip();

        match indices.len() {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(format!("multiple vks for id {}", id,)),
        }
    }

    pub fn contains_vk(&self, vk: &T::VK) -> Result<bool, String> {
        let (_, vks): (Vec<_>, Vec<T::VK>) = self
            .vks
            .clone()
            .into_iter()
            .filter(|(_i, vk_i)| vk == vk_i)
            .map(|(i, vk)| (i, vk))
            .unzip();

        match vks.len() {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(format!("multiple instances of vk {:?}", vk,)),
        }
    }
}

pub struct TrustedKeygenOutput<T: Dvrf> {
    pub pp: DvrfPubParam<T>,
    pub sks: Vec<(usize, T::SK)>,
    pub committee_sk: Option<T::SK>,
}

pub trait Dvrf: Sized {
    type SK;
    type VK: Eq + Clone + Debug;
    type PK;
    type Peval: Clone;
    type Out;

    fn keygen(config: &DvrfConfig) -> TrustedKeygenOutput<Self>;

    fn part_eval(inp: &[u8], sk: &Self::SK, vk: &Self::VK) -> Self::Peval;

    fn part_ver(inp: &[u8], vk: &Self::VK, peval: &Self::Peval) -> bool;

    fn aggregate(threshold: usize, pevals: Vec<(usize, Self::Peval)>) -> Result<Self::Out, String>;

    fn aggregate_with_config(
        config: &DvrfConfig,
        pevals: Vec<(usize, Self::Peval)>,
    ) -> Result<Self::Out, String> {
        Self::aggregate(config.t, pevals)
    }

    fn aggregate_with_pp(
        pp: &DvrfPubParam<Self>,
        pevals: Vec<(usize, Self::Peval)>,
    ) -> Result<Self::Out, String> {
        Self::aggregate(pp.config.t, pevals)
    }

    fn out_ver_pk(inp: &[u8], pk: &Self::PK, out: &Self::Out) -> bool;
    fn out_ver_pp(inp: &[u8], pp: &DvrfPubParam<Self>, out: &Self::Out) -> bool {
        Self::out_ver_pk(inp, &pp.pk, out)
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::{Dvrf, DvrfConfig};
    use crate::{TEST_STRING_1, TEST_STRING_2};
    use rand::prelude::SliceRandom;
    use rand::thread_rng;

    pub fn test_dvrf_partial_evals<T: Dvrf>() {
        let config = DvrfConfig::default();
        let keys = T::keygen(&config);

        let pp = keys.pp;
        let sks = keys.sks;

        assert_eq!(config.n, sks.len());

        for i in 0..config.n {
            let (id, sk_i) = &sks[i];
            let vk_i = pp
                .vk_from_id(*id)
                .expect("(id, vk) should appear exactly once")
                .expect("(id, vk) should appear exactly once");
            let peval = T::part_eval(TEST_STRING_1.as_bytes(), &sk_i, &vk_i);
            assert!(T::part_ver(TEST_STRING_1.as_bytes(), &vk_i, &peval));
            assert!(!T::part_ver(TEST_STRING_2.as_bytes(), &vk_i, &peval));
            let (next_i, _) = sks[(i + 1) % config.n];
            let next_vk = pp
                .vk_from_id(next_i)
                .expect("(id, vk) should appear exactly once")
                .expect("(id, vk) should appear exactly once");
            assert!(!T::part_ver(TEST_STRING_1.as_bytes(), &next_vk, &peval));
        }
    }

    pub fn test_dvrf_aggregation_success<T: Dvrf>() {
        let config = DvrfConfig::default();
        let keys = T::keygen(&config);

        let pp = keys.pp;
        let sks = keys.sks;

        assert_eq!(config.n, sks.len());

        let mut pevals = Vec::new();

        for i in 0..config.n {
            let (id, sk_i) = &sks[i];
            let vk_i = pp
                .vk_from_id(*id)
                .expect("(id, vk) should appear exactly once")
                .expect("(id, vk) should appear exactly once");
            let peval = T::part_eval(TEST_STRING_1.as_bytes(), &sk_i, &vk_i);
            pevals.push((*id, peval));
        }

        let agg_sig = T::aggregate_with_config(&config, pevals.clone())
            .expect("aggregation should not throw error when threshold nodes available");

        assert!(T::out_ver_pk(TEST_STRING_1.as_bytes(), &pp.pk, &agg_sig));
        assert!(T::out_ver_pp(TEST_STRING_1.as_bytes(), &pp, &agg_sig));

        pevals.shuffle(&mut thread_rng());
        pevals.truncate(config.t);

        let agg_sig_subcommittee = T::aggregate_with_config(&config, pevals)
            .expect("aggregation should not throw error when threshold nodes available");

        assert!(T::out_ver_pk(
            TEST_STRING_1.as_bytes(),
            &pp.pk,
            &agg_sig_subcommittee
        ));
        assert!(T::out_ver_pp(
            TEST_STRING_1.as_bytes(),
            &pp,
            &agg_sig_subcommittee
        ));
    }

    pub fn test_dvrf_aggregation_failure_insufficient_pevals<T: Dvrf>() {
        let config = DvrfConfig::default();
        let keys = T::keygen(&config);

        let pp = keys.pp;
        let sks = keys.sks;

        assert_eq!(config.n, sks.len());

        let mut pevals = Vec::new();

        for i in 0..config.t - 1 {
            let (id, sk_i) = &sks[i];

            let vk_i = pp
                .vk_from_id(*id)
                .expect("(id, vk) should appear exactly once")
                .expect("(id, vk) should appear exactly once");
            let peval = T::part_eval(TEST_STRING_1.as_bytes(), &sk_i, &vk_i);
            pevals.push((*id, peval));
        }
        assert!(T::aggregate_with_config(&config, pevals).is_err());
    }
    pub fn test_dvrf_aggregation_failure_invalid_pevals<T: Dvrf>() {
        let config = DvrfConfig::default();

        let keys = T::keygen(&config);

        let pp = keys.pp;
        let sks = keys.sks;
        assert_eq!(config.n, sks.len());

        let mut pevals = Vec::new();

        for i in 0..config.t - 1 {
            let (id, sk_i) = &sks[i];
            let vk_i = pp
                .vk_from_id(*id)
                .expect("(id, vk) should appear exactly once")
                .expect("(id, vk) should appear exactly once");
            let peval = T::part_eval(TEST_STRING_1.as_bytes(), &sk_i, &vk_i);
            pevals.push((*id, peval));
        }

        let (bad_id, bad_sk_i) = &sks[config.t - 1];
        let bad_vk_i = pp
            .vk_from_id(*bad_id)
            .expect("(id, vk) should appear exactly once")
            .expect("(id, vk) should appear exactly once");
        let bad_peval = T::part_eval(TEST_STRING_2.as_bytes(), &bad_sk_i, &bad_vk_i);
        pevals.push((*bad_id, bad_peval));

        let agg_sig = T::aggregate_with_config(&config, pevals)
            .expect("aggregation should not throw error when threshold nodes available");

        assert!(!T::out_ver_pk(TEST_STRING_1.as_bytes(), &pp.pk, &agg_sig));
        assert!(!T::out_ver_pp(TEST_STRING_1.as_bytes(), &pp, &agg_sig));
    }
}
