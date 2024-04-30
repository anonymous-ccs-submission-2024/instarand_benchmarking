use std::time::Duration;

use criterion::measurement::Measurement;
use criterion::BatchSize::SmallInput;
use criterion::{criterion_group, criterion_main, BenchmarkGroup, Criterion, Throughput};
use instarand_benchmarking::impls::GoldbergVrf;
use instarand_benchmarking::random_input;
use instarand_benchmarking::types::vrf::Vrf;

criterion_main!(benches);
criterion_group!(benches, bench_vrf);

fn bench_vrf(criterion: &mut Criterion) {
    let group_name = "bench_vrf";
    let group = &mut criterion.benchmark_group(group_name);
    group.throughput(Throughput::Elements(1)); // each iteration signs one message
    group.measurement_time(Duration::from_secs(10));
    //0.95 is default
    //group.confidence_level(0.95);
    group.sample_size(20);

    bench_vrf_generic::<GoldbergVrf, _>(group, "goldberg_secp256k1");
}

fn bench_vrf_generic<T: Vrf, M: Measurement>(group: &mut BenchmarkGroup<'_, M>, name: &str) {
    bench_vrf_eval::<T, M>(group, &format!("{}/evaluation", name));
    bench_vrf_ver::<T, M>(group, &format!("{}/verification", name));
}

pub fn bench_vrf_eval<T: Vrf, M: Measurement>(group: &mut BenchmarkGroup<'_, M>, id: &str) {
    group.bench_function(id, |bench| {
        bench.iter_batched(
            || {
                let (sk, pk) = T::keygen();
                let inp = random_input();
                (inp, sk, pk)
            },
            |(inp, sk, pk)| T::eval(&inp, &sk, &pk),
            SmallInput,
        )
    });
}
pub fn bench_vrf_ver<T: Vrf, M: Measurement>(group: &mut BenchmarkGroup<'_, M>, id: &str) {
    group.bench_function(id, |bench| {
        bench.iter_batched(
            || {
                let (sk, pk) = T::keygen();
                let inp = random_input();

                let out = T::eval(&inp, &sk, &pk);

                (inp, pk, out)
            },
            |(inp, pk, out)| T::ver(&inp, &pk, &out),
            SmallInput,
        )
    });
}
