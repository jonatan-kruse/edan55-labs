use treewidth::{path_to_graph_tree, the_algorithm};

fn main() {
    // let (_graph, tree) = path_to_graph_tree("./data/eppstein.td");
    // let (_graph, tree) = path_to_graph_tree("./data/ErreraGraph.td");
    let (_graph, tree) = path_to_graph_tree("./data/HarborthGraph");
    println!("\nsize: {:?}", tree.size());
    println!("edges: {:?}", tree.edges());
    println!("max depth: {:?}", tree.max_depth());
    tree.display();

    let answer = the_algorithm("./data/HarborthGraph");
    println!("answer: {}", answer);
}
