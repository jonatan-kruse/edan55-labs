use std::fs;

use independentset::*;
fn main() {
    let file_path = format!("./data/g4.in");
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let (nodes, edges) = parse_input(&input);
    println!("{}", r2((1 << nodes) - 1, &edges));
    r0_test();
    r1_test();
}

fn test() {
    // u_edge = string_to_edge("1010")
    // let z_edge = edges[u_index as usize] | edges[w_index as usize];
    // new_edges[u_index as usize] |= z_edge;
    // new_edges[w_index as usize] &= 0;
    // let new_state = state & !(v + w);
    // return 1 + r2(new_state, &new_edges);
}

fn r0_test() {
    for i in 3..7 {
        let file_path = format!("./data/g{}0.in", i);
        let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
        let (nodes, edges) = parse_input(&input);
        println!("R0, g{}0.in: {}", i, r0((1 << nodes) - 1, &edges));
        calls();
    }
}

fn r1_test() {
    for i in 3..13 {
        let file_path = format!("./data/g{}0.in", i);
        let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
        let (nodes, edges) = parse_input(&input);
        println!("R1, g{}0.in: {}", i, r1((1 << nodes) - 1, &edges));
        calls();
    }
}

fn r2_test() {
    for i in 4..5 {
        let file_path = format!("./data/g{}0.in", i);
        let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
        let (nodes, edges) = parse_input(&input);
        println!("R2, g{}0.in: {}", i, r2((1 << nodes) - 1, &edges));
    }
}
