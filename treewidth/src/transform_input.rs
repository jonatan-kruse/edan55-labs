use crate::arena_tree::ArenaTree;
use core::str;
use std::collections::{HashMap, HashSet};

pub fn parse_graph(input: &str) -> Graph {
    let data = input.lines();
    let mut nodes = HashMap::new();
    data.for_each(|line| {
        if line.starts_with('p') {
            let p = line.split_whitespace().skip(2).next().unwrap();
            let node_count = p.parse().unwrap();
            for key in 1..=node_count {
                nodes.entry(key).or_default();
            }
        } else if !(line.starts_with('c')) {
            parse_graph_line(line, &mut nodes);
        }
    });
    nodes
}

fn parse_graph_line(line: &str, nodes: &mut Graph) {
    let (u, v) = line.split_once(' ').unwrap();
    let u = u.parse().unwrap();
    let v = v.parse().unwrap();
    nodes.entry(u).or_default().insert(v);
    nodes.entry(v).or_default().insert(u);
}

pub fn parse_tree(input: &str) -> ArenaTree<Bag> {
    let data = input.lines();
    let mut tree = ArenaTree::default();
    let mut nodes = HashMap::new();

    data.for_each(|line| match line.chars().next().unwrap() {
        'c' | 's' => (),
        'b' => node_from_line(line, &mut tree),
        _ => parse_graph_line(line, &mut nodes),
    });

    // select a random node as root
    if tree.arena.len() > 0 {
        let &root = tree.arena.keys().next().unwrap();
        tree.arena.get_mut(&root).unwrap().parent = None;
        build_tree(&mut tree, root, &nodes);
    }
    tree
}

fn build_tree(tree: &mut ArenaTree<Bag>, idx: usize, nodes: &Graph) {
    if let Some(neighbors) = nodes.get(&idx) {
        for &neighbor in neighbors {
            if tree.arena.get(&idx).unwrap().children.contains(&neighbor)
                || tree.arena.get(&neighbor).unwrap().parent.is_some()
                || tree.arena.get(&idx).unwrap().parent == Some(neighbor)
            {
                continue;
            }

            tree.arena.get_mut(&neighbor).unwrap().parent = Some(idx);
            tree.arena.get_mut(&idx).unwrap().children.push(neighbor);

            build_tree(tree, neighbor, nodes);
        }
    }
}

fn node_from_line(line: &str, tree: &mut ArenaTree<Bag>) {
    let mut numbs = line.split_ascii_whitespace().skip(1);
    let id = numbs.next().unwrap().parse().unwrap();
    let bag = numbs
        .collect::<HashSet<_>>()
        .iter()
        .enumerate()
        .map(|(i, c)| (c.parse().unwrap(), i.try_into().unwrap()))
        .collect::<HashMap<GlobalIndex, LocalIndex>>();
    tree.node(id, bag);
}

pub type Graph = HashMap<usize, HashSet<usize>>;

//VT is a HashMap<IndexGlobal, IndexLocal>

pub type GlobalIndex = usize;
pub type LocalIndex = u8;
pub type Bitmap = u64;
pub type Score = u32;

pub type Bag = HashMap<GlobalIndex, LocalIndex>;
