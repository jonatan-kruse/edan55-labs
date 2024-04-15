use std::fs;

use independentset::*;
fn main() {
    r0_test();
    r1_test();
    r2_test();
}

fn r0_test() {
    for i in 3..7 {
        let file_path = format!("./data/g{}0.in", i);
        let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
        let (nodes, edges) = parse_input(&input);
        println!("R0, g{}0.in: {}", i, r0((1 << nodes) - 1, &edges));
        // calls();
    }
}

fn r1_test() {
    for i in 3..11 {
        let file_path = format!("./data/g{}0.in", i);
        let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
        let (nodes, edges) = parse_input(&input);
        println!("R1, g{}0.in: {}", i, r1((1 << nodes) - 1, &edges));
        // calls();
    }
}

fn r2_test() {
    for i in 3..13 {
        let file_path = format!("./data/g{}0.in", i);
        let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
        let (nodes, edges) = parse_input(&input);
        println!("R2, g{}0.in: {}", i, r2((1 << nodes) - 1, &edges));
    }
}
