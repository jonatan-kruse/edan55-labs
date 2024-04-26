use std::collections::{HashMap, HashSet};

use crate::arena_tree::ArenaTree;
use crate::transform_input::{Bag, Bitmap, GlobalIndex, Graph, LocalIndex, Score};

type Table = HashMap<Bitmap, Score>;

pub fn solve_max_independent_set(graph: Graph, mut tree: ArenaTree<Bag>) -> Score {
    let root = tree.root().unwrap().idx;
    let (root_table, _) = traverse(&mut tree, &graph, root);
    *root_table.values().max().unwrap()
}

pub fn traverse<'a>(
    tree: &'a ArenaTree<Bag>,
    graph: &'a Graph,
    nodeidx: usize,
) -> (Table, &'a Bag) {
    let children = tree.arena.get(&nodeidx).unwrap().children.clone();
    if children.len() == 0 {
        let node = tree.arena.get(&nodeidx).unwrap();
        let test = (build_leaf_table(graph, &node.val), &node.val);
        // dbg!(test.clone());
        test
    } else {
        let child_tables = children.iter().map(|c| traverse(tree, graph, *c)).collect();
        let node = tree.arena.get(&nodeidx).unwrap();
        let test = (
            build_parent_table(graph, &node.val, child_tables),
            &node.val,
        );
        // dbg!(test.clone());
        test
    }
}

pub fn build_leaf_table(graph: &Graph, bag: &Bag) -> Table {
    let matrix = local_adj_matrix(graph, &bag.vt);
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

pub fn build_parent_table(graph: &Graph, bag: &Bag, child_tables: Vec<(Table, &Bag)>) -> Table {
    let matrix = local_adj_matrix(graph, &bag.vt);
    let mut table = HashMap::new();
    let all_u = 0 as Bitmap..(1 << bag.vt.len());
    for u in all_u {
        let mut independent = true;
        for (bit_position, edges) in matrix.iter().enumerate() {
            if u & (1 << bit_position) != 0 {
                let collisions = u & edges;
                independent &= collisions == 0;
            }
        }
        if independent {
            let mut sum = 0;
            for (child_t, child_bag) in child_tables.clone() {
                let mut max = 0;
                let global_u = localbitmap_to_globalindexs(u, bag);
                let u_intersect_vti = globalindexs_to_localbitmap(&global_u, &child_bag.vt);
                let mask = globalindexs_to_localbitmap(
                    &bag.vt.keys().map(|i| *i).collect(),
                    &child_bag.vt,
                );
                // Kollar bara aktiva inte vilka som ska vara av.
                // Kolla vilka index från U som finns i VT och lås dom.
                // Dvs kolla up hur man iterar subset effektivt

                for ui in 0..(1 << child_bag.vt.len()) {
                    if ui & mask == u_intersect_vti {
                        if let Some(some_score) = child_t.get(&ui) {
                            let global_ui = localbitmap_to_globalindexs(ui, &child_bag);
                            let u_intersect_ui =
                                global_ui.intersection(&global_u).collect::<HashSet<_>>();
                            max = max.max(some_score - u_intersect_ui.len() as u32);
                        }
                    }
                }
                sum += max;
            }
            table.insert(u, sum + u.count_ones());
        }
    }
    table
}

pub fn localbitmap_to_globalindexs(bitmap: Bitmap, bag: &Bag) -> HashSet<GlobalIndex> {
    let mut indexs = vec![];
    for i in 0..bag.vt.len() {
        if bitmap & (1 << i) != 0 {
            indexs.push(i as u8);
        }
    }
    bag.vt
        .clone()
        .into_iter()
        .filter(|(_, local)| indexs.contains(&local))
        .map(|i| i.0)
        .collect()
}

pub fn globalindexs_to_localbitmap(
    subset: &HashSet<GlobalIndex>,
    bag: &HashMap<GlobalIndex, LocalIndex>,
) -> Bitmap {
    let mut bitmap = 0;
    for e in subset.intersection(&bag.keys().map(|u| *u).collect::<HashSet<GlobalIndex>>()) {
        bitmap |= 1 << bag.get(e).unwrap();
    }
    bitmap
}

pub fn local_adj_matrix(graph: &Graph, bag: &HashMap<GlobalIndex, LocalIndex>) -> Vec<Bitmap> {
    let mut matrix = vec![0; bag.len()];
    for (global, local) in bag.iter() {
        matrix[*local as usize] = globalindexs_to_localbitmap(graph.get(global).unwrap(), bag);
    }
    matrix
}
