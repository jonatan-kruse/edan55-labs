use criterion::{criterion_group, criterion_main, Criterion};

/// run with `cargo bench`
fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Treewidth");
    // group.sample_size(500);
    // group.warm_up_time(std::time::Duration::from_secs(5));
    // group.measurement_time(std::time::Duration::from_secs(20));

    group.bench_function(
        "r0 for g40",
        |b: &mut criterion::Bencher<'_, criterion::measurement::WallTime>| b.iter(|| {}),
    );

    group.bench_function("r1 for g70", |b| b.iter(|| {}));

    group.bench_function("r2 for g80", |b| b.iter(|| {}));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
