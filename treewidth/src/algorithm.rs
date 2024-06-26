use std::collections::{HashMap, HashSet};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::arena_tree::ArenaTree;
use crate::transform_input::{Bag, Bitmap, GlobalIndex, Graph, LocalIndex, Score};

type Table = HashMap<Bitmap, Score>;

pub fn solve_max_independent_set(graph: Graph, tree: ArenaTree<Bag>) -> Score {
    let root = tree.root().unwrap().idx;
    let (root_table, _) = traverse(&tree, &graph, root);
    *root_table.values().max().unwrap()
}

pub fn traverse<'a>(
    tree: &'a ArenaTree<Bag>,
    graph: &'a Graph,
    node_idx: usize,
) -> (Table, &'a Bag) {
    let children = tree.arena.get(&node_idx).unwrap().children.clone();
    if children.is_empty() {
        let node = tree.arena.get(&node_idx).unwrap();

        (build_leaf_table(graph, &node.val), &node.val)
    } else {
        let child_tables = children.iter().map(|c| traverse(tree, graph, *c)).collect();
        let node = tree.arena.get(&node_idx).unwrap();

        (
            build_parent_table(graph, &node.val, child_tables),
            &node.val,
        )
    }
}

pub fn build_leaf_table(graph: &Graph, bag: &Bag) -> Table {
    let matrix = local_adj_matrix(graph, bag);
    let mut table = HashMap::new();
    let all_possible_bitmaps = 0 as Bitmap..(1 << bag.len());
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
    let matrix = local_adj_matrix(graph, bag);
    let all_u = 0 as Bitmap..(1 << bag.len());

    all_u
        .into_par_iter()
        .filter_map(|u| {
            let mut independent = true;
            for (bit_position, edges) in matrix.iter().enumerate() {
                if u & (1 << bit_position) != 0 {
                    let collisions = u & edges;
                    independent &= collisions == 0;
                }
            }
            if independent {
                let mut sum = 0;
                for (child_t, child_bag) in child_tables.iter() {
                    let mut max = 0;
                    let global_u = local_bitmap_to_global_indexes(u, bag);
                    let u_intersect_vti = global_indexes_to_local_bitmap(&global_u, child_bag);
                    let mask =
                        global_indexes_to_local_bitmap(&bag.keys().copied().collect(), child_bag);
                    // Kollar bara aktiva inte vilka som ska vara av.
                    // Kolla vilka index från U som finns i VT och lås dom.
                    // Dvs kolla up hur man iterar subset effektivt

                    for ui in 0..(1 << child_bag.len()) {
                        if ui & mask == u_intersect_vti {
                            if let Some(some_score) = child_t.get(&ui) {
                                let global_ui = local_bitmap_to_global_indexes(ui, child_bag);
                                let u_intersect_ui =
                                    global_ui.intersection(&global_u).collect::<HashSet<_>>();
                                max = max.max(some_score - u_intersect_ui.len() as u32);
                            }
                        }
                    }
                    sum += max;
                }
                Some((u, sum + u.count_ones()))
            } else {
                None
            }
        })
        .collect()
}

pub fn local_bitmap_to_global_indexes(bitmap: Bitmap, bag: &Bag) -> HashSet<GlobalIndex> {
    let mut indexs = vec![];
    for i in 0..bag.len() {
        if bitmap & (1 << i) != 0 {
            indexs.push(i as u8);
        }
    }
    bag.iter()
        .filter(|(_, local)| indexs.contains(local))
        .map(|i| *i.0)
        .collect()
}

pub fn global_indexes_to_local_bitmap(
    subset: &HashSet<GlobalIndex>,
    bag: &HashMap<GlobalIndex, LocalIndex>,
) -> Bitmap {
    let mut bitmap = 0;
    for e in subset.intersection(&bag.keys().copied().collect::<HashSet<GlobalIndex>>()) {
        bitmap |= 1 << bag.get(e).unwrap();
    }
    bitmap
}

pub fn local_adj_matrix(graph: &Graph, bag: &HashMap<GlobalIndex, LocalIndex>) -> Vec<Bitmap> {
    let mut matrix = vec![0; bag.len()];
    for (global, local) in bag.iter() {
        matrix[*local as usize] = global_indexes_to_local_bitmap(graph.get(global).unwrap(), bag);
    }
    matrix
}

// Brute force graph search
pub fn r2(state: u128, edges: &[u128]) -> u32 {
    if state == 0 {
        return 0;
    }
    let mut max = 0;
    let mut max_i = 0;
    for (i, &e) in edges.iter().enumerate() {
        let v = 1 << i;
        if v & state == 0 {
            continue;
        }
        let active_edges = state & e;
        let deg = active_edges.count_ones();
        if deg == 2 {
            let u_index = active_edges.trailing_zeros() as usize;
            let u = 1 << u_index;
            let u_edges = edges[u_index];

            let w_index = 127 - active_edges.leading_zeros() as usize;
            let w = 1 << w_index;
            let w_edges = edges[w_index];

            if u_edges & w != 0 {
                return 1 + r2(state & !(u + w + v), edges);
            } else {
                let mut new_edges = edges.to_vec();
                // set an edge from u to all nodes that w has an edge to
                new_edges[u_index] |= w_edges;
                // set an edge to u from all nodes that have an edge to w
                for (index, number) in new_edges.iter_mut().enumerate() {
                    if w_edges & (1 << index) != 0 {
                        *number |= u;
                    }
                }
                let new_state = state & !(v | w);
                return 1 + r2(new_state, &new_edges);
            }
        }
        if deg == 1 {
            return 1 + r2((state & !v) & !edges[i], edges);
        }
        if deg == 0 {
            return 1 + r2(state & !v, edges);
        }
        if deg > max {
            max = deg;
            max_i = i;
        }
    }

    let new_state = state & !(1 << max_i);
    let results = rayon::join(
        || r2(new_state, edges),
        || 1 + r2(new_state & !edges[max_i], edges),
    );
    results.0.max(results.1)
}
