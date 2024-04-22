use std::borrow::BorrowMut;
use std::collections::VecDeque;
use std::hash::Hash;
use std::{
    collections::{HashMap, HashSet},
    fs,
    sync::atomic::{AtomicUsize, Ordering},
};

static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn calls() {
    println!("Calls: {}", CALL_COUNT.load(Ordering::SeqCst));
    CALL_COUNT.fetch_and(0, Ordering::SeqCst);
}

pub fn parse_graph(input: &str) -> HashMap<usize, HashSet<usize>> {
    let data = input.lines();
    let mut nodes = HashMap::new();
    data.for_each(|line| {
        if !(line.starts_with('p') || line.starts_with('c')) {
            parse_graph_line(line, &mut nodes);
        }
    });
    nodes
}

fn parse_graph_line(line: &str, nodes: &mut HashMap<usize, HashSet<usize>>) {
    let (u, v) = line.split_once(' ').unwrap();
    let u = u.parse().unwrap();
    let v = v.parse().unwrap();
    nodes.entry(u).or_insert_with(HashSet::new).insert(v);
    nodes.entry(v).or_insert_with(HashSet::new).insert(u);
}

fn parse_tree(input: &str) -> TreeNode {
    let data = input.lines();
    let mut nodes = HashMap::new();
    let mut tree_nodes = data
        .filter_map(|line| match line.chars().next().unwrap() {
            'c' | 's' => None,
            'b' => Some(TreeNode::from_line(line)),
            _ => {
                parse_graph_line(line, &mut nodes);
                None
            }
        })
        .collect::<HashMap<_, _>>();
    let root = tree_nodes.values().next().unwrap();
    root.clone()
}

#[derive(Debug, Clone)]
pub struct TreeNode {
    id: usize,
    v_t: HashSet<usize>,
    table: HashMap<usize, usize>,
    children: HashSet<TreeNode>,
}

impl Hash for TreeNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for TreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for TreeNode {}

impl TreeNode {
    fn new(v_t: HashSet<usize>, id: usize) -> Self {
        Self {
            id,
            v_t,
            table: HashMap::new(),
            children: HashSet::new(),
        }
    }

    fn from_line(line: &str) -> (usize, Self) {
        let mut chars = line.chars().skip(1);
        let id = chars.next().unwrap().to_digit(10).unwrap() as usize;
        let v_t = chars
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<HashSet<_>>();
        (id, Self::new(v_t, id))
    }
}

pub fn path_to_tree(path: &str) -> TreeNode {
    let input = fs::read_to_string(path).expect("Should have been able to read the file");
    parse_tree(&input)
}

pub fn path_to_graph(path: &str) -> HashMap<usize, HashSet<usize>> {
    let input = fs::read_to_string(path).expect("Should have been able to read the file");
    parse_graph(&input)
}
