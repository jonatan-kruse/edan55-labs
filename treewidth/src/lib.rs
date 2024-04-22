use std::{
    fs,
    sync::atomic::{AtomicUsize, Ordering},
};

static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn calls() {
    println!("Calls: {}", CALL_COUNT.load(Ordering::SeqCst));
    CALL_COUNT.fetch_and(0, Ordering::SeqCst);
}

pub fn parse_input(input: &str) -> (i32, Vec<u128>) {
    let mut data = input.lines();
    let nodes = data.next().unwrap().parse().unwrap();
    let edges = data.map(string_to_edge).collect::<Vec<_>>();
    (nodes, edges)
}

pub fn string_to_edge(s: &str) -> u128 {
    u128::from_str_radix(&s.replace(' ', "").chars().rev().collect::<String>(), 2).unwrap()
}

pub fn path_to_edges(path: &str) -> (i32, Vec<u128>) {
    let input = fs::read_to_string(path).expect("Should have been able to read the file");
    parse_input(&input)
}
