use independentset::*;
fn main() {
    r0_test();
    r1_test();
    r2_test();
}

fn r0_test() {
    for i in 3..7 {
        let (nodes, edges) = path_to_edges(&format!("./data/g{}0.in", i));
        println!("R0, g{}0.in: {}", i, r0((1 << nodes) - 1, &edges));
    }
}

fn r1_test() {
    for i in 3..11 {
        let (nodes, edges) = path_to_edges(&format!("./data/g{}0.in", i));
        println!("R1, g{}0.in: {}", i, r1((1 << nodes) - 1, &edges));
    }
}

fn r2_test() {
    for i in 3..13 {
        let (nodes, edges) = path_to_edges(&format!("./data/g{}0.in", i));
        println!("R2, g{}0.in: {}", i, r2((1 << nodes) - 1, &edges));
    }
}

// fn r0_test() {
//     (3..7).into_par_iter().for_each(|i| {
//         let (nodes, edges) = path_to_edges(&format!("./data/g{}0.in", i));
//         println!("R0, g{}0.in: {}", i, r0((1 << nodes) - 1, &edges));
//     });
// }

// fn r1_test() {
//     (3..13).into_par_iter().for_each(|i| {
//         let (nodes, edges) = path_to_edges(&format!("./data/g{}0.in", i));
//         println!("R1, g{}0.in: {}", i, r1((1 << nodes) - 1, &edges));
//     });
// }

// fn r2_test() {
//     (3..13).into_par_iter().for_each(|i| {
//         let (nodes, edges) = path_to_edges(&format!("./data/g{}0.in", i));
//         println!("R2, g{}0.in: {}", i, r2((1 << nodes) - 1, &edges));
//     });
// }
