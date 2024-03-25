use rand::Rng;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashSet;

pub fn random_cut(nbr_of_nodes: i32, edges: &[Edge]) -> (HashSet<i32>, i32) {
    let mut rng = rand::thread_rng();
    let cut = (1..=nbr_of_nodes)
        .filter(|_| rng.gen_bool(0.5))
        .collect::<HashSet<_>>();
    let score = score_cut(&cut, edges);
    (cut, score)
}

pub fn greedy_swap_cut(
    start_cut: HashSet<i32>,
    edges: &[Edge],
    nbr_of_nodes: i32,
) -> (HashSet<i32>, i32) {
    let mut cut = start_cut;
    let mut best_score = score_cut(&cut, edges);
    let mut improved = true;

    while improved {
        improved = false;
        for n in 0..=nbr_of_nodes {
            if !cut.remove(&n) {
                cut.insert(n);
            }
            let test_score = score_cut(&cut, edges);
            if test_score > best_score {
                improved = true;
                best_score = test_score;
            } else if !cut.remove(&n) {
                cut.insert(n);
            }
        }
    }
    (cut, best_score)
}

fn score_cut(cut: &HashSet<i32>, edges: &[Edge]) -> i32 {
    edges
        .par_iter()
        .filter_map(|e| {
            let u_in_cut = cut.contains(&e.u);
            let v_in_cut = cut.contains(&e.v);
            if u_in_cut ^ v_in_cut {
                Some(e.w)
            } else {
                None
            }
        })
        .sum()
}

#[derive(Debug)]
pub struct Edge {
    u: i32,
    v: i32,
    w: i32,
}

pub fn parse_input(input: &str) -> (i32, i32, Vec<Edge>) {
    let mut data = input.lines();
    let (nodes, edges) = data.next().unwrap().split_once(' ').unwrap();
    let nbr_of_nodes: i32 = nodes.parse().unwrap();
    let nbr_of_edges: i32 = edges.parse().unwrap();
    let edges = data.map(string_to_edge).collect::<Vec<_>>();
    (nbr_of_nodes, nbr_of_edges, edges)
}

fn string_to_edge(s: &str) -> Edge {
    let mut iter = s.split_whitespace();
    let u = iter.next().unwrap().parse().unwrap();
    let v = iter.next().unwrap().parse().unwrap();
    let w = iter.next().unwrap().parse().unwrap();
    Edge { u, v, w }
}
