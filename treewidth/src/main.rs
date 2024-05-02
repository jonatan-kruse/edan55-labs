use std::thread;
use treewidth::{print_bag_tree, run_with_spinner, the_algorithm};

fn main() {
    
    // let path = "web4";
    // let path = "WorldMap";
    // let path = "FibonacciTree_10";
    // let path = "StarGraph_100";
    // let path = "TutteGraph";
    let path = "DorogovtsevGoltsevMendesGraph";
    // let path = "HanoiTowerGraph_4_3";

    print_bag_tree(path);
    println!("------ Maximum Independent Set ------");
    let answer = run_with_spinner(|| the_algorithm(path));
    println!("{answer}");
    // run_all();
}

fn run_all() {
    let paths = include_str!("filenames.txt")
        .lines()
        .map(|s| format!("{}", s))
        .collect::<Vec<_>>();
    for p in paths {
        print!("{}: ", p);
        println!("{}", the_algorithm(&p));
    }
}
