use std::time::Duration;

use criterion::measurement::Measurement;
use criterion::BatchSize::SmallInput;
use criterion::{criterion_group, criterion_main, BenchmarkGroup, Criterion, Throughput};
use instarand_benchmarking::impls::flexirand_glow::g1_to_bytes;
use instarand_benchmarking::impls::{BlsVrfHashless, GlowDvrfHashless, GoldbergVrf};
use instarand_benchmarking::types::dvrf::{Dvrf, DvrfConfig};
use instarand_benchmarking::types::instarand::InstarandClient;
use instarand_benchmarking::types::vrf::Vrf;

criterion_main!(benches);
criterion_group!(benches, bench_instarand);

fn bench_instarand(criterion: &mut Criterion) {
    let group_name = "bench_instarand";
    let group = &mut criterion.benchmark_group(group_name);
    group.throughput(Throughput::Elements(1)); // each iteration signs one message
    group.measurement_time(Duration::from_secs(10));
    //0.95 is default
    //group.confidence_level(0.95);
    group.sample_size(20);

    bench_instarand_client::<GoldbergVrf, _>(group, "goldberg_vrf_secp256k1");

    bench_instarand_server_centralized::<GoldbergVrf, BlsVrfHashless, _>(group, "bls_vrf_bn254");

    bench_instarand_server_distributed::<GoldbergVrf, GlowDvrfHashless, _>(
        group,
        "glow_dvrf_bn254",
    );
}
fn bench_instarand_client<T: InstarandClient, M: Measurement>(
    group: &mut BenchmarkGroup<'_, M>,
    name: &str,
) {
    bench_instarand_client_keygen_goldberg::<T, M>(group, name);
    bench_instarand_client_vrf_eval_goldberg::<T, M>(group, name);
    bench_instarand_client_vrf_ver_goldberg::<T, M>(group, name);
}

fn bench_instarand_server_centralized<C: InstarandClient, S: Vrf, M: Measurement>(
    group: &mut BenchmarkGroup<'_, M>,
    name: &str,
) {
    bench_instarand_server_vrf_eval::<C, S, M>(group, name);
    bench_instarand_server_vrf_ver::<C, S, M>(group, name);
}

fn bench_instarand_server_distributed<C: InstarandClient, S: Dvrf, M: Measurement>(
    group: &mut BenchmarkGroup<'_, M>,
    name: &str,
) {
    bench_instarand_server_dvrf_partial_eval::<C, S, M>(group, name);
    bench_instarand_server_dvrf_partial_ver::<C, S, M>(group, name);
    bench_instarand_server_dvrf_aggregate::<C, S, M>(group, name);
    bench_instarand_server_dvrf_verify::<C, S, M>(group, name);
}

pub fn bench_instarand_client_keygen_goldberg<T: InstarandClient, M: Measurement>(
    group: &mut BenchmarkGroup<'_, M>,
    name: &str,
) {
    let id = format!("{}/keygen", name);

    group.bench_function(id, |bench| bench.iter(T::keygen));
}

pub fn bench_instarand_client_vrf_eval_goldberg<T: InstarandClient, M: Measurement>(
    group: &mut BenchmarkGroup<'_, M>,
    name: &str,
) {
    let id = format!("{}/client_vrf_eval", name);
    let u = Vec::new();
    let mut i = 1_usize;

    let (sk_s, pk_s) = BlsVrfHashless::keygen();

    group.bench_function(id, |bench| {
        bench.iter_batched(
            || {
                let (sk, pk) = T::keygen();
                let mut x = T::generate_server_input(&u, &pk);
                // recompute y each time since new pk
                let y = BlsVrfHashless::eval(&x, &sk_s, &pk_s);
                x.append(&mut g1_to_bytes(&y).to_vec());
                i += 1;
                x.append(&mut i.to_be_bytes().to_vec());
                (sk, pk, x)
            },
            |(sk, pk, client_prefix)| {
                let client_out = T::eval(&client_prefix, &sk, &pk);
                T::hash_output(&client_prefix, &client_out);
            },
            SmallInput,
        )
    });
}
pub fn bench_instarand_client_vrf_ver_goldberg<T: InstarandClient, M: Measurement>(
    group: &mut BenchmarkGroup<'_, M>,
    name: &str,
) {
    let id = format!("{}/client_vrf_ver", name);
    let u = Vec::new();
    let mut i = 1_usize;

    let (sk_s, pk_s) = BlsVrfHashless::keygen();

    group.bench_function(id, |bench| {
        bench.iter_batched(
            || {
                let (sk, pk) = T::keygen();
                let mut x = T::generate_server_input(&u, &pk);
                // recompute y each time since new pk
                let y = BlsVrfHashless::eval(&x, &sk_s, &pk_s);
                // client can precompute input but still needs to append i each time
                x.append(&mut g1_to_bytes(&y).to_vec());
                i += 1;
                x.append(&mut i.to_be_bytes().to_vec());

                let client_out = T::eval(&x, &sk, &pk);
                let z_i = T::hash_output(&x, &client_out);

                (pk, x, client_out, z_i)
            },
            |(pk, client_prefix, client_out, z_i)| {
                T::ver(&client_prefix, &pk, &client_out)
                    && z_i == T::hash_output(&client_prefix, &client_out)
            },
            SmallInput,
        )
    });
}

pub fn bench_instarand_server_vrf_eval<C: InstarandClient, S: Vrf, M: Measurement>(
    group: &mut BenchmarkGroup<'_, M>,
    name: &str,
) {
    let id = format!("{}/server_vrf_eval", name);
    let u = Vec::new();

    group.bench_function(id, |bench| {
        bench.iter_batched(
            || {
                let (sk_s, pk_s) = S::keygen();
                let (_, pk_c) = C::keygen();
                let inp = C::generate_server_input(&u, &pk_c);

                (inp, sk_s, pk_s)
            },
            |(inp, sk, pk)| S::eval(&inp, &sk, &pk),
            SmallInput,
        )
    });
}
pub fn bench_instarand_server_vrf_ver<C: InstarandClient, S: Vrf, M: Measurement>(
    group: &mut BenchmarkGroup<'_, M>,
    name: &str,
) {
    let id = format!("{}/server_vrf_ver", name);
    let u = Vec::new();

    group.bench_function(id, |bench| {
        bench.iter_batched(
            || {
                let (sk_s, pk_s) = S::keygen();
                let (_, pk_c) = C::keygen();
                let inp = C::generate_server_input(&u, &pk_c);
                let out = S::eval(&inp, &sk_s, &pk_s);

                (inp, pk_s, out)
            },
            |(inp, pk, out)| S::ver(&inp, &pk, &out),
            SmallInput,
        )
    });
}

pub fn bench_instarand_server_dvrf_partial_eval<C: InstarandClient, S: Dvrf, M: Measurement>(
    group: &mut BenchmarkGroup<'_, M>,
    name: &str,
) {
    let id = format!("{}/server_dvrf_partial_eval", name);
    let u = Vec::new();
    let config = DvrfConfig::default();

    group.bench_function(id, |bench| {
        bench.iter_batched(
            || {
                let keys = S::keygen(&config);
                let (_, pk_c) = C::keygen();
                let inp = C::generate_server_input(&u, &pk_c);

                (inp, keys)
            },
            |(inp, keys)| {
                let sk = &keys.sks[0].1;
                let vk = &keys.pp.vks[0].1;
                S::part_eval(&inp, sk, vk)
            },
            SmallInput,
        )
    });
}
pub fn bench_instarand_server_dvrf_partial_ver<C: InstarandClient, S: Dvrf, M: Measurement>(
    group: &mut BenchmarkGroup<'_, M>,
    name: &str,
) {
    let id = format!("{}/server_dvrf_partial_ver", name);
    let u = Vec::new();
    let config = DvrfConfig::default();

    group.bench_function(id, |bench| {
        bench.iter_batched(
            || {
                let keys = S::keygen(&config);
                let (_, pk_c) = C::keygen();
                let inp = C::generate_server_input(&u, &pk_c);

                let sk = &keys.sks[0].1;
                let vk = &keys.pp.vks[0].1;
                let peval = S::part_eval(&inp, sk, vk);

                (inp, vk.clone(), peval)
            },
            |(inp, vk, peval)| S::part_ver(&inp, &vk, &peval),
            SmallInput,
        )
    });
}

pub fn bench_instarand_server_dvrf_aggregate<C: InstarandClient, S: Dvrf, M: Measurement>(
    group: &mut BenchmarkGroup<'_, M>,
    name: &str,
) {
    let u = Vec::new();

    let mut i = 8;
    while i <= 64 {
        let id = format!("{}_threshold_{}", name, i);
        let config = DvrfConfig { t: i, n: i };

        group.bench_function(id, |bench| {
            bench.iter_batched(
                || {
                    let (_, pk_c) = C::keygen();
                    let inp = C::generate_server_input(&u, &pk_c);

                    let keys = S::keygen(&config);

                    let mut pevals = Vec::new();

                    let pp = keys.pp;
                    let sks = keys.sks;

                    for i in 0..config.t {
                        let (id, sk_i) = &sks[i];
                        let vk_i = pp
                            .vk_from_id(*id)
                            .expect("(id, vk) should appear exactly once")
                            .expect("(id, vk) should appear exactly once");
                        let peval = S::part_eval(&inp, sk_i, &vk_i);
                        pevals.push((*id, peval));
                    }
                    pevals
                },
                |pevals| {
                    S::aggregate_with_config(&config, pevals)
                        .expect("aggregation should not fail with honest evaluations")
                },
                SmallInput,
            )
        });
        i *= 2;
    }
}

pub fn bench_instarand_server_dvrf_verify<C: InstarandClient, S: Dvrf, M: Measurement>(
    group: &mut BenchmarkGroup<'_, M>,
    name: &str,
) {
    let id = format!("{}/server_dvrf_verify", name);
    let u = Vec::new();
    let config = DvrfConfig::default();

    group.bench_function(id, |bench| {
        bench.iter_batched(
            || {
                let (_, pk_c) = C::keygen();
                let inp = C::generate_server_input(&u, &pk_c);

                let keys = S::keygen(&config);

                let mut pevals = Vec::new();

                let pp = keys.pp;
                let sks = keys.sks;

                for i in 0..config.t {
                    let (ind, sk_i) = &sks[i];
                    let vk_i = pp
                        .vk_from_id(*ind)
                        .expect("(ind, vk) should appear exactly once")
                        .expect("(ind, vk) should appear exactly once");
                    let peval = S::part_eval(&inp, sk_i, &vk_i);
                    pevals.push((*ind, peval));
                }
                let out = S::aggregate_with_config(&config, pevals)
                    .expect("aggregation should not fail with honest evaluations");

                (inp, pp.pk, out)
            },
            |(inp, pk, out)| S::out_ver_pk(&inp, &pk, &out),
            SmallInput,
        )
    });
}
