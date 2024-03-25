use std::collections::HashSet;

use criterion::{criterion_group, criterion_main, Criterion};
use maxcut::{greedy_swap_cut, parse_input, random_cut};

// run with `cargo bench`
fn criterion_benchmark(c: &mut Criterion) {
    let weighted = include_str!("../data/pw09_100.9.txt");
    let (nbr_of_nodes, _, edges) = parse_input(weighted);

    c.bench_function("s weighted", |b| {
        b.iter(|| greedy_swap_cut(HashSet::new(), &edges, nbr_of_nodes))
    });

    c.bench_function("rs weighted", |b| {
        b.iter(|| greedy_swap_cut(random_cut(nbr_of_nodes, &edges).0, &edges, nbr_of_nodes))
    });

    let matching = include_str!("../data/matching_1000.txt");
    let (nbr_of_nodes, _, edges) = parse_input(matching);

    c.bench_function("s matching", |b| {
        b.iter(|| greedy_swap_cut(HashSet::new(), &edges, nbr_of_nodes))
    });

    c.bench_function("rs matching", |b| {
        b.iter(|| greedy_swap_cut(random_cut(nbr_of_nodes, &edges).0, &edges, nbr_of_nodes))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
