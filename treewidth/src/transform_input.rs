use crate::arena_tree::ArenaTree;
use core::str;
use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
};

pub fn parse_graph(input: &str) -> Graph {
    let data = input.lines();
    let mut nodes = HashMap::new();
    data.for_each(|line| {
        if !(line.starts_with('p') || line.starts_with('c')) {
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

pub fn parse_tree(input: &str) -> ArenaTree<VT> {
    let data = input.lines();
    let mut tree = ArenaTree::default();
    let mut nodes = HashMap::new();

    data.for_each(|line| match line.chars().next().unwrap() {
        'c' | 's' => (),
        'b' => node_from_line(line, &mut tree),
        _ => parse_graph_line(line, &mut nodes),
    });

    // select a random node as root
    let &root = tree.arena.keys().next().unwrap();
    tree.arena.get_mut(&root).unwrap().parent = None;
    build_tree(&mut tree, root, &nodes);
    tree
}

fn build_tree(tree: &mut ArenaTree<VT>, idx: usize, nodes: &Graph) {
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

fn node_from_line(line: &str, tree: &mut ArenaTree<VT>) {
    let mut numbs = line.split_ascii_whitespace().skip(1);
    let id = numbs.next().unwrap().parse().unwrap();
    let v_t = numbs.map(|c| c.parse().unwrap()).collect::<HashSet<_>>();
    tree.node(id, VT(v_t));
}

pub type Graph = HashMap<usize, HashSet<usize>>;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct VT(HashSet<usize>);

impl Display for VT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        // loop over the set of vertices and print them with a space between them but not at the end
        for (i, v) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }
        // print the closing brace
        write!(f, "}}")?;
        Ok(())
    }
}
