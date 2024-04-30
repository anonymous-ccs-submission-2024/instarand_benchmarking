use std::time::Duration;

use criterion::BatchSize::SmallInput;
use criterion::{
    criterion_group, criterion_main, measurement::Measurement, BenchmarkGroup, Criterion,
    Throughput,
};
use instarand_benchmarking::impls::GlowDvrf;
use instarand_benchmarking::random_input;
use instarand_benchmarking::types::dvrf::{Dvrf, DvrfConfig};

criterion_main!(benches);
criterion_group!(benches, bench_dvrf);

fn bench_dvrf(criterion: &mut Criterion) {
    let group_name = "bench_glow_dvrf";
    let group = &mut criterion.benchmark_group(group_name);
    group.throughput(Throughput::Elements(1)); // each iteration signs one message
    group.measurement_time(Duration::from_secs(10));
    //0.95 is default
    //group.confidence_level(0.95);
    group.sample_size(20);

    bench_dvrf_generic::<GlowDvrf, _>(group, "glow_dvrf");
}

fn bench_dvrf_generic<T: Dvrf, M: Measurement>(group: &mut BenchmarkGroup<'_, M>, name: &str) {
    bench_dvrf_partial_eval::<T, M>(group, format!("{}/partial_evaluation", name).as_str());
    bench_dvrf_partial_ver::<T, M>(group, format!("{}/partial_verification", name).as_str());
    bench_dvrf_aggregate::<T, M>(group, format!("{}/aggregation", name).as_str());
    bench_dvrf_verify::<T, M>(group, format!("{}/verify", name).as_str());
}

pub fn bench_dvrf_partial_eval<T: Dvrf, M: Measurement>(
    group: &mut BenchmarkGroup<'_, M>,
    id: &str,
) {
    let config = DvrfConfig::default();

    group.bench_function(id, |bench| {
        bench.iter_batched(
            || {
                let keys = T::keygen(&config);
                let inp = random_input();
                (inp, keys)
            },
            |(inp, keys)| {
                let sk = &keys.sks[0].1;
                let vk = &keys.pp.vks[0].1;
                T::part_eval(&inp, sk, vk)
            },
            SmallInput,
        )
    });
}
pub fn bench_dvrf_partial_ver<T: Dvrf, M: Measurement>(
    group: &mut BenchmarkGroup<'_, M>,
    id: &str,
) {
    let config = DvrfConfig::default();

    group.bench_function(id, |bench| {
        bench.iter_batched(
            || {
                let keys = T::keygen(&config);
                let inp = random_input();
                let sk = &keys.sks[0].1;
                let vk = &keys.pp.vks[0].1;
                let peval = T::part_eval(&inp, sk, vk);
                (inp, vk.clone(), peval)
            },
            |(inp, vk, peval)| T::part_ver(&inp, &vk, &peval),
            SmallInput,
        )
    });
}

pub fn bench_dvrf_aggregate<T: Dvrf, M: Measurement>(
    group: &mut BenchmarkGroup<'_, M>,
    id_prefix: &str,
) {
    let mut i = 8;
    while i <= 64 {
        let id = format!("{}_threshold_{}", id_prefix, i);
        let config = DvrfConfig { t: i, n: i };
        group.bench_function(id, |bench| {
            bench.iter_batched(
                || {
                    let keys = T::keygen(&config);

                    let inp = random_input();

                    let mut pevals = Vec::new();

                    let pp = keys.pp;
                    let sks = keys.sks;

                    for i in 0..config.t {
                        let (id, sk_i) = &sks[i];
                        let vk_i = pp
                            .vk_from_id(*id)
                            .expect("(id, vk) should appear exactly once")
                            .expect("(id, vk) should appear exactly once");
                        let peval = T::part_eval(&inp, sk_i, &vk_i);
                        pevals.push((*id, peval));
                    }
                    pevals
                },
                |pevals| {
                    T::aggregate_with_config(&config, pevals)
                        .expect("aggregation should not fail with honest evaluations");
                },
                SmallInput,
            )
        });
        i *= 2;
    }
}

pub fn bench_dvrf_verify<T: Dvrf, M: Measurement>(group: &mut BenchmarkGroup<'_, M>, id: &str) {
    let config = DvrfConfig::default();

    group.bench_function(id, |bench| {
        bench.iter_batched(
            || {
                let keys = T::keygen(&config);

                let inp = random_input();

                let mut pevals = Vec::new();

                let pp = keys.pp;
                let sks = keys.sks;

                for i in 0..config.t {
                    let (ind, sk_i) = &sks[i];
                    let vk_i = pp
                        .vk_from_id(*ind)
                        .expect("(ind, vk) should appear exactly once")
                        .expect("(ind, vk) should appear exactly once");
                    let peval = T::part_eval(&inp, sk_i, &vk_i);
                    pevals.push((*ind, peval));
                }
                let out = T::aggregate_with_config(&config, pevals)
                    .expect("aggregation should not fail with honest evaluations");

                (inp, pp.pk, out)
            },
            |(inp, pk, out)| T::out_ver_pk(&inp, &pk, &out),
            SmallInput,
        )
    });
}
