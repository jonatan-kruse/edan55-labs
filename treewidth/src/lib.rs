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

pub fn the_algorithm(graph_name: &str) -> Score {
    let (graph, tree) = path_to_graph_tree(&("./data/".to_owned() + graph_name));
    solve_max_independent_set(graph, tree)
}

fn path_to_graph_tree(path: &str) -> (Graph, ArenaTree<Bag>) {
    let graph = path_to_graph(&(path.to_owned() + ".gr"));
    let tree = path_to_tree(&(path.to_owned() + ".td"));
    (graph, tree)
}

pub fn print_bag_tree(path: &str) {
    let tree = path_to_tree(&(path.to_owned() + ".td"));
    println!("--------------- Info ----------------");
    println!("size: {:?}", tree.size());
    println!("edges: {:?}", tree.edges());
    println!("max depth: {:?}", tree.max_depth());
    println!("--------------- Tree ----------------");
    tree.display();
}

pub fn run_with_spinner<T>(func: impl FnOnce() -> T) -> T {
    let spinner_chars = [
        "🌕\n", "🌕\n", "🌔\n", "🌓\n", "🌒\n", "🌑\n", "🌑\n", "🌘\n", "🌗\n", "🌖\n",
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
