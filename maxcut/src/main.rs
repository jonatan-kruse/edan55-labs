use rand::Rng;
use std::collections::HashSet;
fn main() {
    let _matching = include_str!("../data/matching_1000.txt").lines();
    let _weighted = include_str!("../data/pw09_100.9.txt").lines();
    // let mut data = _matching;
    let mut data = _weighted;
    let (nodes, edges) = data.next().unwrap().split_once(' ').unwrap();
    let nbr_of_nodes: i32 = nodes.parse().unwrap();
    let nbr_of_edges: i32 = edges.parse().unwrap();
    let edges = data.map(string_to_edge).collect::<Vec<_>>();
    println!("Nodes: {}, Edges: {}\n", nbr_of_nodes, nbr_of_edges);

    // R-algo
    let random = random_cut(nbr_of_nodes);
    println!("Random cut score: {}", score_cut(&random, &edges));

    // S-algo
    let greedy = greedy_swap_cut(HashSet::new(), &edges, nbr_of_nodes);
    println!("Greedy cut score: {}", score_cut(&greedy, &edges));

    // RS_algo
    let random_greedy = greedy_swap_cut(random_cut(nbr_of_nodes), &edges, nbr_of_nodes);
    println!("Random start greedy cut score: {}", score_cut(&random_greedy, &edges));
}

fn random_cut(nbr_of_nodes: i32) -> HashSet<i32> {
    let mut rng = rand::thread_rng();
    return (1..=nbr_of_nodes)
        .filter(|_| rng.gen_bool(0.5))
        .collect::<HashSet<_>>();
}

fn greedy_swap_cut(start_cut: HashSet<i32>, edges: &Vec<Edge>, nbr_of_nodes: i32) -> HashSet<i32> {
    let mut old_cut = start_cut;
    let mut old_score = score_cut(&old_cut, &edges);
    'iter: loop {
        let mut complete = true;
        for n in 0..=nbr_of_nodes {
            let mut testcut = old_cut.clone();
            if testcut.contains(&n) {
                testcut.remove(&n);
            } else {
                testcut.insert(n);
            }
            let testscore = score_cut(&testcut, &edges);
            if testscore > old_score {
                complete = false;
                old_cut = testcut;
                old_score = testscore;
            }
        }
        if complete {
            break 'iter;
        }
    }
    return old_cut;
}

fn string_to_edge(s: &str) -> Edge {
    let mut iter = s.split_whitespace();
    let u = iter.next().unwrap().parse().unwrap();
    let v = iter.next().unwrap().parse().unwrap();
    let w = iter.next().unwrap().parse().unwrap();
    Edge { u, v, w }
}

fn score_cut(cut: &HashSet<i32>, edges: &Vec<Edge>) -> i32 {
    return edges
        .iter()
        .filter(|e| cut.contains(&e.u) ^ cut.contains(&e.v))
        .fold(0, |sum, e| sum + &e.w);
}

#[derive(Debug)]
struct Edge {
    u: i32,
    v: i32,
    w: i32,
}
