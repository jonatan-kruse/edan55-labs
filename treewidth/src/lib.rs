mod algorithm;
mod arena_tree;
mod transform_input;
use crossterm::{cursor, ExecutableCommand};
use std::io::stdout;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time::Duration};

use std::fs;

use algorithm::solve_max_independent_set;
use algorithm::r2;
use arena_tree::ArenaTree;
use transform_input::{parse_graph, parse_tree, Bag, Graph, Score};

fn path_to_tree(path: &str) -> ArenaTree<Bag> {
    let input = fs::read_to_string(path).expect("Should have been able to read the file");
    parse_tree(&input)
}

fn path_to_graph(path: &str) -> Graph {
    let input = fs::read_to_string(path).expect("Should have been able to read the file");
    parse_graph(&input)
}


pub fn get_nodes_and_width(path: &str) -> (usize, usize){
    let new_path = "./data/".to_owned() + path;
    let (graph, tree) = path_to_graph_tree(&new_path);
    let nodes = graph.len();
    let width = tree.arena.iter().fold(0, |acc, node|acc.max(node.1.val.len() - 1));
    (nodes,width)
}

pub fn the_algorithm(path: &str) -> Score {
    let new_path = "./data/".to_owned() + path;
    let (graph, tree) = path_to_graph_tree(&new_path);
    if graph.len() == 0{
        return 0;
    }
    if graph.len() < 1 {
        let edges = graph_to_adj_matrix(graph);
        r2((1 << edges.len()) - 1, &edges)
    } else {
        solve_max_independent_set(graph, tree)
    }
}

fn path_to_graph_tree(path: &str) -> (Graph, ArenaTree<Bag>) {
    let graph = path_to_graph(&(path.to_owned() + ".gr"));
    let tree = path_to_tree(&(path.to_owned() + ".td"));
    (graph, tree)
}

fn graph_to_adj_matrix(graph: Graph) -> Vec<u128> {
    let mut matrix = vec![0; graph.len()];
    for (key, edges) in graph {
        let mut row = 0;
        for node in edges {
            row += 1 << (node - 1);
        }
        matrix[key-1] = row;
    }
    matrix
}

pub fn print_bag_tree(path: &str) {
    let new_path = "./data/".to_owned() + path;
    let tree = path_to_tree(&(new_path + ".td"));
    println!("--------------- Info ----------------");
    println!("size: {:?}", tree.size());
    println!("edges: {:?}", tree.edges());
    println!("max depth: {:?}", tree.max_depth());
    println!("--------------- Tree ----------------");
    tree.display();
}

pub fn run_with_spinner<T>(func: impl FnOnce() -> T) -> T {
    let spinner_chars = [
        "🌕  ", "🌕  ", "🌔  ", "🌓  ", "🌒  ", "🌑  ", "🌑  ", "🌘  ", "🌗  ", "🌖  ",
    ];
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    // keep track of what was the cursor position
    stdout().execute(cursor::SavePosition).unwrap();

    let handle = std::thread::spawn(move || {
        let mut index = 0;
        while running_clone.load(Ordering::SeqCst) {
            print!("{}", spinner_chars[index]);
            // let _ = stdout().flush();
            index = (index + 1) % spinner_chars.len();
            thread::sleep(Duration::from_millis(100));
            // move cursor to the beginning of the line and up one line
            stdout().execute(cursor::RestorePosition).unwrap();
        }
    });

    let result = func();

    running.store(false, Ordering::SeqCst);
    handle.join().unwrap();

    result
}
