fn main() {
    let _matching = include_str!("../data/matching_1000.txt").lines();
    let _weighted = include_str!("../data/pw09_100.9.txt").lines();
    let mut data = _matching;
    let (nodes, edges) = data.next().unwrap().split_once(' ').unwrap();
    let nbr_of_nodes: i32 = nodes.parse().unwrap();
    let nbr_of_edges: i32 = edges.parse().unwrap();
    let edges = data.map(string_to_edge).collect::<Vec<_>>();
    dbg!(edges, nbr_of_nodes, nbr_of_edges);
}

fn string_to_edge(s: &str) -> Edge {
    let mut iter = s.split_whitespace();
    let u = iter.next().unwrap().parse().unwrap();
    let v = iter.next().unwrap().parse().unwrap();
    let w = iter.next().unwrap().parse().unwrap();
    Edge { u, v, w }
}

#[derive(Debug)]
struct Edge {
    u: i32,
    v: i32,
    w: i32,
}

