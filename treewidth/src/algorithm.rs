use std::collections::{HashMap, HashSet};

use crate::arena_tree::ArenaTree;
use crate::transform_input::{Graph, VT};

pub fn solve_max_independent_set(graph: Graph, tree: ArenaTree<VT>) -> usize {
    let mut max = 0;
    for bag in tree.arena.values() {
        let table = build_table(&graph, &bag.val);
        dbg!(table.clone());
        max = max.max(*table.values().max().unwrap());
    }
    return max;
}

pub fn build_table(graph: &Graph, bag: &VT) -> HashMap<usize, usize> {
    let matrix = local_adj_matrix(&graph, bag);
    (0..(1 << bag.0.len()))
        .filter(|U| 0 == matrix.iter().fold(0, |acc, edge| acc + (edge & U)))
        .map(|U| (U, U.count_ones() as usize))
        .collect::<HashMap<usize, usize>>()
}

pub fn nodes_to_bitmap(nodes: &HashSet<usize>, bag: &VT) -> usize {
    let mut edges = 0;
    for e in nodes {
        println!("{}", bag.0.get(e).unwrap());
        edges |= 1 << bag.0.get(e).unwrap();
    }

    edges
}

pub fn local_adj_matrix(graph: &Graph, bag: &VT) -> Vec<usize> {
    let mut matrix = vec![0; bag.0.len()];
    for (global, local) in bag.0.iter() {
        matrix[*local] = nodes_to_bitmap(graph.get(&global).unwrap(), bag);
    }
    matrix
}
