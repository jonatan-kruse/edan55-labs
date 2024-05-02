use criterion::{criterion_group, criterion_main, Criterion};

/// run with `cargo bench`
fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("FedUPS");

    // group.sample_size(500);
    // group.warm_up_time(std::time::Duration::from_secs(5));
    // group.measurement_time(std::time::Duration::from_secs(20));

    group.bench_function(
        "Test",
        |b: &mut criterion::Bencher<'_, criterion::measurement::WallTime>| {
            b.iter(|| {
                // the_algorithm(path);
            })
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
