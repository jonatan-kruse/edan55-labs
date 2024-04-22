use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
    fs,
};

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
    nodes.entry(u).or_default().insert(v);
    nodes.entry(v).or_default().insert(u);
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct VT(HashSet<usize>);

fn parse_tree(input: &str) -> ArenaTree<VT> {
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

fn build_tree(tree: &mut ArenaTree<VT>, idx: usize, nodes: &HashMap<usize, HashSet<usize>>) {
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

pub fn path_to_tree(path: &str) -> ArenaTree<VT> {
    let input = fs::read_to_string(path).expect("Should have been able to read the file");
    parse_tree(&input)
}

pub fn path_to_graph(path: &str) -> HashMap<usize, HashSet<usize>> {
    let input = fs::read_to_string(path).expect("Should have been able to read the file");
    parse_graph(&input)
}

#[derive(Debug)]
struct Node<T>
where
    T: PartialEq,
{
    idx: usize,
    #[allow(dead_code)]
    val: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            parent: None,
            children: vec![],
        }
    }
}

#[derive(Debug, Default)]
pub struct ArenaTree<T>
where
    T: PartialEq,
{
    arena: HashMap<usize, Node<T>>,
}

impl<T> ArenaTree<T>
where
    T: PartialEq + Display,
{
    fn node(&mut self, idx: usize, val: T) -> usize {
        self.arena.entry(idx).or_insert_with(|| Node::new(idx, val));
        idx
    }

    pub fn display(&self) {
        for node in self.arena.values() {
            if node.parent.is_none() {
                self.display_node(node.idx, 0, "", true);
                break;
            }
        }
    }

    fn display_node(&self, idx: usize, depth: usize, prefix: &str, is_last: bool) {
        let node = self.arena.get(&idx).unwrap();

        if depth > 0 {
            let connector = if is_last { "╚═ " } else { "╠═ " };
            println!("{}{}Bag: {}", prefix, connector, node.idx);
        } else {
            println!("Bag: {}", node.idx);
        }

        let children_count = node.children.len();
        for (i, &child_idx) in node.children.iter().enumerate() {
            let last_child = i == children_count - 1;
            let new_prefix = if depth > 0 {
                let continuation = if is_last { "    " } else { "║   " };
                format!("{}{}", prefix, continuation)
            } else {
                String::from("")
            };

            self.display_node(child_idx, depth + 1, &new_prefix, last_child);
        }
    }

    pub fn size(&self) -> usize {
        self.arena.len()
    }

    pub fn edges(&self) -> usize {
        self.arena
            .iter()
            .fold(0, |acc, (_, node)| acc + node.children.len())
    }

    pub fn depth(&self, idx: usize) -> usize {
        match self.arena.get(&idx).unwrap().parent {
            Some(id) => 1 + self.depth(id),
            None => 0,
        }
    }

    pub fn max_depth(&self) -> usize {
        self.arena
            .iter()
            .fold(0, |acc, (idx, _)| acc.max(1 + self.depth(*idx)))
    }
}
