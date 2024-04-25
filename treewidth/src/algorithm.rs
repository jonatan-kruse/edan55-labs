use std::collections::{HashMap, HashSet};

use crate::arena_tree::ArenaTree;
use crate::transform_input::{Bag, Bitmap, GlobalIndex, Graph, Score};

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
    (0 as Bitmap..(1 << bag.vt.len()))
        .filter(|u| 0 == matrix.iter().fold(0, |acc, edge| acc + (edge & u)))
        .map(|u| (u as u64, u.count_ones() as Score))
        .collect::<HashMap<Bitmap, Score>>()
}

pub fn globalindexs_to_localbitmap(subset: &HashSet<GlobalIndex>, bag: &Bag) -> Bitmap {
    let mut bitmap = 0;
    for e in subset.intersection(&bag.vt.keys().map(|u| *u).collect::<HashSet<GlobalIndex>>()) {
        bitmap |= 1 << bag.vt.get(e).unwrap();
    }
    bitmap
}

pub fn local_adj_matrix(graph: &Graph, bag: &Bag) -> Vec<Bitmap> {
    let mut matrix = vec![0; bag.vt.len()];
    for (global, local) in bag.vt.iter() {
        matrix[*local as usize] = globalindexs_to_localbitmap(graph.get(global).unwrap(), bag);
    }
    matrix
}
