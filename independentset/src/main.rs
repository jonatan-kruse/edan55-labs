use independentset::*;
fn main() {
    r0_test();
    r1_test();
    r2_test();
}

fn r0_test() {
    println!();
    println!("R0:");
    for i in 3..8 {
        let (nodes, edges) = path_to_edges(&format!("./data/g{}0.in", i));
        println!("g{}0.in: {}", i, r0((1 << nodes) - 1, &edges));
    }
}

fn r1_test() {
    println!();
    println!("R1:");
    for i in 3..11 {
        let (nodes, edges) = path_to_edges(&format!("./data/g{}0.in", i));
        println!("g{}0.in: {}", i, r1((1 << nodes) - 1, &edges));
    }
}

fn r2_test() {
    println!();
    println!("R2:");
    for i in 3..13 {
        let (nodes, edges) = path_to_edges(&format!("./data/g{}0.in", i));
        println!("g{}0.in: {}", i, r2((1 << nodes) - 1, &edges));
    }
}
