use std::fs;

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
