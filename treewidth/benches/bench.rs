use criterion::{criterion_group, criterion_main, Criterion};
use treewidth::the_algorithm;

/// run with `cargo bench`
fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Treewidth");
    // let path = "./data/TutteGraph";
    let path = "./data/HanoiTowerGraph_4_3"; // width: 13

    // group.sample_size(500);
    // group.warm_up_time(std::time::Duration::from_secs(5));
    // group.measurement_time(std::time::Duration::from_secs(20));

    group.bench_function(
        "HanoiTowerGraph_4_3",
        |b: &mut criterion::Bencher<'_, criterion::measurement::WallTime>| {
            b.iter(|| {
                the_algorithm(path);
            })
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
