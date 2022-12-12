use std::collections::HashSet;

pub fn solve() {

    let mut start_of_packet: usize = 0;

    let tree: Tree<String> = Tree::default();
    tree.node

    if let Ok(lines) = crate::helpers::read_lines("./data/7.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
            }
        }
    }
    println!("Start of packet: {}", start_of_packet);
}

#[derive(Debug, Default)]
struct Tree<T>
    where
        T: PartialEq
{
    arena: Vec<Node<T>>,
}

#[derive(Debug)]
struct Node<T>
    where
        T: PartialEq
{
    idx: usize,
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

impl<T> Tree<T>
    where
        T: PartialEq,
{
    fn node(&mut self, val: T) -> usize {
        //first see if it exists
        for node in &self.arena {
            if node.val == val {
                return node.idx;
            }
        }
        // Otherwise, add new node
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val));
        idx
    }
}