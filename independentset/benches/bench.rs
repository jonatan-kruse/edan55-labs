use criterion::{criterion_group, criterion_main, Criterion};
use independentset::{path_to_edges, r0, r1, r2};

/// run with `cargo bench`
fn criterion_benchmark(c: &mut Criterion) {
    let (nbr_of_nodes_r0, edges_r0) = path_to_edges("./data/g40.in");
    let (nbr_of_nodes_r1, edges_r1) = path_to_edges("./data/g70.in");
    let (nbr_of_nodes_r2, edges_r2) = path_to_edges("./data/g80.in");

    let mut group = c.benchmark_group("Independent set");
    // group.sample_size(500);
    // group.warm_up_time(std::time::Duration::from_secs(5));
    // group.measurement_time(std::time::Duration::from_secs(20));

    group.bench_function("r0 for g40", |b: &mut criterion::Bencher<'_, criterion::measurement::WallTime>| {
        b.iter(|| r0((1 << nbr_of_nodes_r0) - 1, &edges_r0))
    });

    group.bench_function("r1 for g70", |b| {
        b.iter(|| r1((1 << nbr_of_nodes_r1) - 1, &edges_r1))
    });

    group.bench_function("r2 for g80", |b| {
        b.iter(|| r2((1 << nbr_of_nodes_r2) - 1, &edges_r2))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
