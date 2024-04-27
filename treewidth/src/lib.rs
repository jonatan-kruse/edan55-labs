mod algorithm;
mod arena_tree;
mod transform_input;
use crossterm::{
    cursor,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::{stdout, Write};
use std::{thread, time::Duration};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

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

pub fn the_algorithm(path: &str) -> Score {
    let (graph, tree) = path_to_graph_tree(path);
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
    let spinner_chars = ["🌕\n", "🌕\n", "🌔\n", "🌓\n", "🌒\n", "🌑\n", "🌑\n", "🌘\n", "🌗\n", "🌖\n"];
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    let handle = std::thread::spawn(move || {
        let mut index = 0;
        while running_clone.load(Ordering::SeqCst) {
            print!("{}", spinner_chars[index]);
            let _ = stdout().flush();
            index = (index + 1) % spinner_chars.len();
            thread::sleep(Duration::from_millis(100));
            // move cursor to the beginning of the line and up one line
            stdout().execute(cursor::MoveToColumn(0)).unwrap();
            stdout().execute(cursor::MoveUp(1)).unwrap();
        }
    });

    let result = func();

    running.store(false, Ordering::SeqCst);
    stop_spinner(handle);
    result
}

fn stop_spinner(handle: std::thread::JoinHandle<()>) {
    handle.join().unwrap();
    let mut stdout = stdout();
    stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
    stdout.execute(cursor::MoveToColumn(0)).unwrap();
}
