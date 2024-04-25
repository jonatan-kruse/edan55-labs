use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
pub struct Node<T> {
    pub idx: usize,
    pub val: T,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

impl<T> Node<T> {
    pub fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            parent: None,
            children: vec![],
        }
    }
}

#[derive(Debug, Default)]
pub struct ArenaTree<T> {
    pub arena: HashMap<usize, Node<T>>,
}

impl<T> ArenaTree<T>
where
    T: Display,
{
    pub fn node(&mut self, idx: usize, val: T) -> usize {
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

    pub fn root(&self) -> Option<&Node<T>> {
        self.arena.values().find(|node| node.parent.is_none())
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
