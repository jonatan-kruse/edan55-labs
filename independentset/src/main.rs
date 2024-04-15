use independentset::*;
fn main() {
    let input = include_str!("../data/g120.in");
    let (nodes, edges) = parse_input(input);
    println!("{}", r1((1 << nodes)-1, &edges))
}
