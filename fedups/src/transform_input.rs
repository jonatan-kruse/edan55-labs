use std::{
    collections::{HashSet, VecDeque},
    fs,
};

pub fn transform_input(path: String) -> Graph {
    let input = fs::read_to_string(path).expect("Should have been able to read the file");
    let mut lines = input.lines();
    let [nodes, _edges, home, fed, post] = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>()[..]
    else {
        panic!("Not enough input");
    };
    let edges = lines
        .map(|line| {
            let mut values = line.split_ascii_whitespace();
            let u = values.next().unwrap().parse::<usize>().unwrap();
            let v = values.next().unwrap().parse::<usize>().unwrap();
            let t = values.next().unwrap().parse::<f32>().unwrap();
            let puv = values.next().unwrap().parse::<f32>().unwrap();
            let pvu = values.next().unwrap().parse::<f32>().unwrap();
            Edge { u, v, t, puv, pvu }
        })
        .collect::<Vec<_>>();
    let mut queue = VecDeque::new();
    let mut found = HashSet::new();
    queue.push_back(home);
    found.insert(home);
    while let Some(node) = queue.pop_front() {
        for e in edges.iter() {
            if (e.u == node) & (e.pvu > 0.0) & (!found.contains(&e.v)) {
                queue.push_back(e.v);
                found.insert(e.v);
            } else if (e.v == node) & (e.puv > 0.0) & (!found.contains(&e.u)) {
                queue.push_back(e.u);
                found.insert(e.u);
            }
        }
    }
    let edges: Vec<Edge> = edges.into_iter().filter(|e| found.contains(&e.u) & found.contains(&e.v)).collect();
    Graph {
        nodes,
        edges,
        home,
        fed,
        post,
    }
}

pub struct Graph {
    pub nodes: usize,
    pub edges: Vec<Edge>,
    pub home: usize,
    pub fed: usize,
    pub post: usize,
}
#[derive(Clone)]
pub struct Edge {
    pub u: usize,
    pub v: usize,
    pub t: f32,
    pub puv: f32,
    pub pvu: f32,
}
