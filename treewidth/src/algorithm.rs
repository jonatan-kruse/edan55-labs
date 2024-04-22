use crate::arena_tree::ArenaTree;
use crate::transform_input::{Graph, VT};

pub fn solve_max_independent_set(graph: Graph, tree: ArenaTree<VT>) -> usize {
    tree.max_depth() + graph.len()
}
