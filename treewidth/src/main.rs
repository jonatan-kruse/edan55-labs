use treewidth::path_to_tree;

fn main() {
    // let tree = path_to_tree("./data/eppstein.td");
    // let tree = path_to_tree("./data/ErreraGraph.td");
    let tree = path_to_tree("./data/HarborthGraph.td");
    println!("\nsize: {:?}", tree.size());
    println!("edges: {:?}", tree.edges());
    println!("max depth: {:?}", tree.max_depth());
    tree.display();
}
