use std::time::Duration;

use criterion::measurement::Measurement;
use criterion::BatchSize::SmallInput;
use criterion::{criterion_group, criterion_main, BenchmarkGroup, Criterion, Throughput};
use instarand_benchmarking::impls::FlexiRandGlow;
use instarand_benchmarking::random_input;
use instarand_benchmarking::types::dvrf::DvrfConfig;
use instarand_benchmarking::types::flexirand::FlexiRand;

criterion_main!(benches);
criterion_group!(benches, bench_flexirand);

fn bench_flexirand(criterion: &mut Criterion) {
    let group_name = "bench_flexirand";
    let group = &mut criterion.benchmark_group(group_name);
    group.throughput(Throughput::Elements(1)); // each iteration signs one message
    group.measurement_time(Duration::from_secs(10));
    //0.95 is default
    //group.confidence_level(0.95);
    group.sample_size(20);

    bench_flexirand_glow::<FlexiRandGlow, _>(group, "flexirand_glow_bn254");
}

fn bench_flexirand_glow<T: FlexiRand, M: Measurement>(
    group: &mut BenchmarkGroup<'_, M>,
    name: &str,
) {
    bench_flexirand_blinding::<T, M>(group, name);
    bench_flexirand_inp_ver::<T, M>(group, name);
    bench_flexirand_partial_eval::<T, M>(group, name);
    bench_flexirand_partial_ver::<T, M>(group, name);
    bench_flexirand_aggregate::<T, M>(group, name);
    bench_flexirand_preverification::<T, M>(group, name);
    bench_flexirand_unblinding::<T, M>(group, name);
    //bench_flexirand_unblinding_with_precompute::<T, M>(group, name);
    bench_flexirand_verification::<T, M>(group, name);
}

pub fn bench_flexirand_blinding<T: FlexiRand, M: Measurement>(
    group: &mut BenchmarkGroup<'_, M>,
    name: &str,
) {
    let id = format!("{}/blinding", name);

    group.bench_function(id, |bench| {
        bench.iter_batched(
            || random_input(),
            |inp| {
                let bf = T::blinding_factor();
                T::blind_input_with_bf(&inp, &bf)
            },
            SmallInput,
        )
    });
}

pub fn bench_flexirand_inp_ver<T: FlexiRand, M: Measurement>(
    group: &mut BenchmarkGroup<'_, M>,
    name: &str,
) {
    let id = format!("{}/input_verifcation", name);
    group.bench_function(id, |bench| {
        bench.iter_batched(
            || {
                let inp = random_input();
                let bf = T::blinding_factor();
                let blinded_inp = T::blind_input_with_bf(&inp, &bf);
                (inp, blinded_inp)
            },
            |(inp, blinded_inp)| T::inp_ver(&inp, &blinded_inp),
            SmallInput,
        )
    });
}

pub fn bench_flexirand_partial_eval<T: FlexiRand, M: Measurement>(
    group: &mut BenchmarkGroup<'_, M>,
    name: &str,
) {
    let id = format!("{}/partial_evaluation", name);
    let config = DvrfConfig::default();

    group.bench_function(id, |bench| {
        bench.iter_batched(
            || {
                let keys = T::keygen(&config);

                let inp = random_input();
                let bf = T::blinding_factor();
                let blinded_inp = T::blind_input_with_bf(&inp, &bf);
                (keys, blinded_inp)
            },
            |(keys, blinded_inp)| {
                let sk = &keys.sks[0].1;
                let vk = &keys.pp.vks[0].1;
                T::part_eval(&blinded_inp, sk, vk)
            },
            SmallInput,
        )
    });
}

pub fn bench_flexirand_partial_ver<T: FlexiRand, M: Measurement>(
    group: &mut BenchmarkGroup<'_, M>,
    name: &str,
) {
    let id = format!("{}/partial_verification", name);
    let config = DvrfConfig::default();

    group.bench_function(id, |bench| {
        bench.iter_batched(
            || {
                let keys = T::keygen(&config);
                let vk = &keys.pp.vks[0].1.clone();
                let sk = &keys.sks[0].1;

                let inp = random_input();
                let bf = T::blinding_factor();
                let blinded_inp = T::blind_input_with_bf(&inp, &bf);

                let peval = T::part_eval(&blinded_inp, sk, vk);
                (blinded_inp, vk.clone(), peval)
            },
            |(blinded_inp, vk, peval)| T::part_ver(&blinded_inp, &vk, &peval),
            SmallInput,
        )
    });
}

pub fn bench_flexirand_aggregate<T: FlexiRand, M: Measurement>(
    group: &mut BenchmarkGroup<'_, M>,
    name: &str,
) {
    let mut i = 8;
    while i <= 64 {
        let id = format!("{}_threshold_{}", name, i);
        let config = DvrfConfig { t: i, n: i };
        group.bench_function(id, |bench| {
            bench.iter_batched(
                || {
                    let keys = T::keygen(&config);
                    let inp = random_input();
                    let bf = T::blinding_factor();
                    let blinded_inp = T::blind_input_with_bf(&inp, &bf);

                    let mut pevals = Vec::new();

                    let pp = keys.pp;
                    let sks = keys.sks;

                    for i in 0..config.t {
                        let (id, sk_i) = &sks[i];
                        let vk_i = pp
                            .vk_from_id(*id)
                            .expect("(id, vk) should appear exactly once")
                            .expect("(id, vk) should appear exactly once");
                        let peval = T::part_eval(&blinded_inp, sk_i, &vk_i);
                        pevals.push((*id, peval));
                    }
                    pevals
                },
                |pevals| T::aggregate_with_config(&config, pevals).expect(""),
                SmallInput,
            )
        });
        i *= 2;
    }
}

// FLEXIRAND prever is just a pairing, not even a hash
pub fn bench_flexirand_preverification<T: FlexiRand, M: Measurement>(
    group: &mut BenchmarkGroup<'_, M>,
    name: &str,
) {
    let id = format!("{}/prever", name);
    let config = DvrfConfig::default();

    group.bench_function(id, |bench| {
        bench.iter_batched(
            || {
                let keys = T::keygen(&config);

                let inp = random_input();
                let bf = T::blinding_factor();
                let blinded_inp = T::blind_input_with_bf(&inp, &bf);

                let mut pevals = Vec::new();

                let pp = keys.pp;
                let sks = keys.sks;

                for i in 0..config.t {
                    let (ind, sk_i) = &sks[i];
                    let vk_i = pp
                        .vk_from_id(*ind)
                        .expect("(ind, vk) should appear exactly once")
                        .expect("(ind, vk) should appear exactly once");
                    let peval = T::part_eval(&blinded_inp, sk_i, &vk_i);
                    pevals.push((*ind, peval));
                }
                let blind_out = T::aggregate_with_config(&config, pevals).expect("");

                (blinded_inp, pp.pk, blind_out)
            },
            |(inp, pk, blinded_out)| T::prever_with_pk(&inp, &pk, &blinded_out),
            SmallInput,
        )
    });
}

// partial eval and aggregate coverd by GLOW, so is pre_ver (double check no extra hashes)
pub fn bench_flexirand_unblinding<T: FlexiRand, M: Measurement>(
    group: &mut BenchmarkGroup<'_, M>,
    name: &str,
) {
    let id = format!("{}/unblinding", name);

    let config = DvrfConfig::default();

    group.bench_function(id, |bench| {
        bench.iter_batched(
            || {
                let keys = T::keygen(&config);

                let inp = random_input();
                let bf = T::blinding_factor();
                let blinded_inp = T::blind_input_with_bf(&inp, &bf);

                let mut pevals = Vec::new();

                let pp = keys.pp;
                let sks = keys.sks;

                for i in 0..config.t {
                    let (ind, sk_i) = &sks[i];
                    let vk_i = pp
                        .vk_from_id(*ind)
                        .expect("(ind, vk) should appear exactly once")
                        .expect("(ind, vk) should appear exactly once");
                    let peval = T::part_eval(&blinded_inp, sk_i, &vk_i);
                    pevals.push((*ind, peval));
                }
                let blind_out = T::aggregate_with_config(&config, pevals).expect("");

                (bf, blind_out)
            },
            |(bf, blind_out)| T::unblind_output(&bf, &blind_out),
            SmallInput,
        )
    });
}

pub fn bench_flexirand_verification<T: FlexiRand, M: Measurement>(
    group: &mut BenchmarkGroup<'_, M>,
    name: &str,
) {
    let id = format!("{}/verification", name);

    let config = DvrfConfig::default();

    group.bench_function(id, |bench| {
        bench.iter_batched(
            || {
                let keys = T::keygen(&config);

                let inp = random_input();
                let bf = T::blinding_factor();
                let blinded_inp = T::blind_input_with_bf(&inp, &bf);

                let mut pevals = Vec::new();

                let pp = keys.pp;
                let sks = keys.sks;

                for i in 0..config.t {
                    let (ind, sk_i) = &sks[i];
                    let vk_i = pp
                        .vk_from_id(*ind)
                        .expect("(ind, vk) should appear exactly once")
                        .expect("(ind, vk) should appear exactly once");
                    let peval = T::part_eval(&blinded_inp, sk_i, &vk_i);
                    pevals.push((*ind, peval));
                }
                let blind_out = T::aggregate_with_config(&config, pevals).expect("");
                let unblinded_out = T::unblind_output(&bf, &blind_out);
                (inp, unblinded_out, pp.pk)
            },
            |(inp, unblinded_out, pk)| T::verify_out_with_pk(&inp, &unblinded_out, &pk),
            SmallInput,
        )
    });
}
