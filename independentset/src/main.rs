use std::collections::HashSet;

use maxcut::{greedy_swap_cut, parse_input, random_cut};
fn main() {
    let matching = include_str!("../data/matching_1000.txt");
    let weighted = include_str!("../data/pw09_100.9.txt");

    println!("\n-------------------------------------------");
    println!("Matching: ");
    test_algos(matching);

    println!("\n-------------------------------------------");
    println!("Weighted: ");
    test_algos(weighted);
}

pub fn test_algos(input: &str) {
    let (nbr_of_nodes, nbr_of_edges, edges) = parse_input(input);
    println!("Nodes: {}, Edges: {}\n", nbr_of_nodes, nbr_of_edges);

    println!("T = 1:");
    // R-algo;
    println!("R score: {}", random_cut(nbr_of_nodes, &edges).1);

    // S-algo
    println!(
        "S score: {}",
        greedy_swap_cut(HashSet::new(), &edges, nbr_of_nodes).1
    );

    // RS_algo
    println!(
        "RS score: {}",
        greedy_swap_cut(random_cut(nbr_of_nodes, &edges).0, &edges, nbr_of_nodes).1
    );

    println!("T = 100:");
    let t = 100;
    let mut r = vec![];
    let s = greedy_swap_cut(HashSet::new(), &edges, nbr_of_nodes).1;
    let mut rs = vec![];

    for _ in 0..t {
        r.push(random_cut(nbr_of_nodes, &edges).1);
        rs.push(greedy_swap_cut(random_cut(nbr_of_nodes, &edges).0, &edges, nbr_of_nodes).1);
    }
    println!(
        "R:  Avg(C): {}, Max(C): {}",
        r.iter().sum::<i32>() / (r.len() as i32),
        r.iter().max().unwrap()
    );
    println!("S:  Avg(C): {}, Max(C): {}", s, s);
    println!(
        "RS: Avg(C): {}, Max(C): {}",
        rs.iter().sum::<i32>() / (rs.len() as i32),
        rs.iter().max().unwrap()
    );

    // For histogram
    // println!("R histogram data:");
    // for c in r {
    //     println!("{}", c);
    // }
    // println!("RS histogram data:");
    // for c in rs {
    //     println!("{}", c);
    // }
}