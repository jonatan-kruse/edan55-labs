mod algorithm;
mod arena_tree;
mod transform_input;

use std::fs;

use algorithm::solve_max_independent_set;
use arena_tree::ArenaTree;
use transform_input::{parse_graph, parse_tree, Graph, Bag};

fn path_to_tree(path: &str) -> ArenaTree<Bag> {
    let input = fs::read_to_string(path).expect("Should have been able to read the file");
    parse_tree(&input)
}

fn path_to_graph(path: &str) -> Graph {
    let input = fs::read_to_string(path).expect("Should have been able to read the file");
    parse_graph(&input)
}

pub fn the_algorithm(path: &str) -> usize {
    let (graph, tree) = path_to_graph_tree(path);
    solve_max_independent_set(graph, tree)
}

pub fn path_to_graph_tree(path: &str) -> (Graph, ArenaTree<Bag>) {
    let graph = path_to_graph(&(path.to_owned() + ".gr"));
    let tree = path_to_tree(&(path.to_owned() + ".td"));
    (graph, tree)
}
