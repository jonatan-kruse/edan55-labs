use std::collections::{HashMap, HashSet};

use crate::arena_tree::ArenaTree;
use crate::transform_input::{Bag, Bitmap, Graph, Score};

pub fn solve_max_independent_set(graph: Graph, tree: ArenaTree<Bag>) -> Score {
    let mut max = 0;
    for bag in tree.arena.values() {
        let table = build_table(&graph, &bag.val);
        dbg!(table.clone());
        max = max.max(*table.values().max().unwrap());
    }
    max
}

pub fn build_table(graph: &Graph, bag: &Bag) -> HashMap<Bitmap, Score> {
    let matrix = local_adj_matrix(graph, bag);
    (0..(1 << bag.vt.len()))
        .filter(|U| 0 == matrix.iter().fold(0, |acc, edge| acc + (edge & U)))
        .map(|U| (U as u64, U.count_ones() as Score))
        .collect::<HashMap<Bitmap, Score>>()
}

pub fn nodes_to_bitmap(nodes: &HashSet<usize>, bag: &Bag) -> usize {
    let mut edges = 0;
    for e in nodes {
        println!("{}", bag.vt.get(e).unwrap());
        edges |= 1 << bag.vt.get(e).unwrap();
    }

    edges
}

pub fn local_adj_matrix(graph: &Graph, bag: &Bag) -> Vec<usize> {
    let mut matrix = vec![0; bag.vt.len()];
    for (global, local) in bag.vt.iter() {
        matrix[*local as usize] = nodes_to_bitmap(graph.get(global).unwrap(), bag);
    }
    matrix
}
