mod arena_tree;
mod transform_input;

use std::{
    collections::{HashMap, HashSet},
    fs,
};

use arena_tree::ArenaTree;
use transform_input::{parse_graph, parse_tree, VT};

pub fn path_to_tree(path: &str) -> ArenaTree<VT> {
    let input = fs::read_to_string(path).expect("Should have been able to read the file");
    parse_tree(&input)
}

pub fn path_to_graph(path: &str) -> HashMap<usize, HashSet<usize>> {
    let input = fs::read_to_string(path).expect("Should have been able to read the file");
    parse_graph(&input)
}
