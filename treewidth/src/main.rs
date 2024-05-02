use treewidth::{get_nodes_and_width, print_bag_tree, run_with_spinner, the_algorithm};

fn main() {
    let path = "HoffmanSingletonGraph";

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
        // web4 & $5$ & $2$ & $3$ \\ 
        let (nodes, width) = get_nodes_and_width(&p);
        //graph
        print!("{} & ", p);
        //nodes
        print!("${}$ & ", nodes);
        //width
        print!("${}$ & ", width);
        //max independent
        println!("${}$ \\\\", the_algorithm(&p));
    }
}
