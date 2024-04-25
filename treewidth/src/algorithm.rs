use std::collections::{HashMap, HashSet};

use crate::arena_tree::{ArenaTree, Node};
use crate::transform_input::{Bag, Bitmap, GlobalIndex, Graph, Score};

pub fn solve_max_independent_set(graph: Graph, mut tree: ArenaTree<Bag>) -> Score {
    let root = tree.root().unwrap().idx;
    traverse(&mut tree, &graph, root);
    let mut max = 0;
    for node in tree.arena.values() {
        let table = &node.val.table;
        dbg!(table.clone());
        max = max.max(*table.values().max().unwrap());
    }
    max
}

pub fn traverse(tree: &mut ArenaTree<Bag>, graph: &Graph, nodeidx: usize) {
    let children = tree.arena.get(&nodeidx).unwrap().children.clone();
    if children.len() == 0 {
        let node = tree.arena.get_mut(&nodeidx).unwrap();
        node.val.set_table(build_leaf_table(graph, &node.val));
    } else {
        for c in children.clone() {
            traverse(tree, graph, c);
        }
        let node = tree.arena.get_mut(&nodeidx).unwrap();
        node.val.set_table(build_parent_table(graph, &node.val));
    }
}

pub fn build_leaf_table(graph: &Graph, bag: &Bag) -> HashMap<Bitmap, Score> {
    let matrix = local_adj_matrix(graph, bag);
    let mut table = HashMap::new();
    let all_possible_bitmaps = 0 as Bitmap..(1 << bag.vt.len());
    for bitmap in all_possible_bitmaps {
        let mut independent = true;
        for (i, e) in matrix.iter().enumerate() {
            if bitmap & (1 << i) != 0 {
                let collisions = bitmap & e;
                independent &= collisions == 0;
            }
        }
        if independent {
            table.insert(bitmap, bitmap.count_ones());
        }
    }
    table
}

pub fn build_parent_table(graph: &Graph, bag: &Bag) -> HashMap<Bitmap, Score> {
    build_leaf_table(graph, bag)
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
