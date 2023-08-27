use crate::source::Source;

use std::cmp::Ordering;
use std::ops::Add;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum NodeType<T>
where
    T: Copy + Add<Output = T> + Eq + Ord,
{
    Leaf(char),
    Branch(Arc<[Node<T>; 2]>),
}

#[derive(Debug, Clone)]
pub struct Node<T>
where
    T: Copy + Add<Output = T> + Eq + Ord,
{
    probability: T,
    node_type: NodeType<T>,
}

impl<T> PartialEq for Node<T>
where
    T: Copy + Add<Output = T> + Eq + Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.probability == other.probability
    }
}

impl<T> Eq for Node<T> where T: Eq + Copy + Add<Output = T> + Ord {}

impl<T> PartialOrd for Node<T>
where
    T: Copy + Add<Output = T> + Eq + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.probability.partial_cmp(&other.probability)
    }
}

impl<T> Ord for Node<T>
where
    T: Copy + Add<Output = T> + Eq + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.probability.cmp(&other.probability)
    }
}

impl<T> Node<T>
where
    T: Copy + Add<Output = T> + Eq + Ord,
{
    pub fn new_leaf(probability: T, symbol: char) -> Node<T> {
        Node {
            probability,
            node_type: NodeType::Leaf(symbol),
        }
    }
    pub fn new_branch(left: Node<T>, right: Node<T>) -> Node<T> {
        Node {
            probability: left.probability + right.probability,
            node_type: NodeType::Branch(Arc::new([left, right])),
        }
    }
    pub fn probability(&self) -> T {
        self.probability
    }
    pub fn node_type(&self) -> &NodeType<T> {
        &self.node_type
    }
    #[allow(dead_code)]
    pub fn same_as(&self, other: &Node<T>) -> bool {
        if self.probability != other.probability {
            return false;
        }
        match (&self.node_type, &other.node_type) {
            (NodeType::Leaf(symbol), NodeType::Leaf(other_symbol)) => symbol == other_symbol,
            (NodeType::Branch(children), NodeType::Branch(other_children)) => {
                children[0].same_as(&other_children[0]) && children[1].same_as(&other_children[1])
            }
            (_, _) => false,
        }
    }
}

impl Node<u32> {
    #[allow(dead_code)]
    pub fn new_huffman(source: &Source<u32>) -> Option<Node<u32>> {
        let mut nodes = source.to_leaves_vec();
        loop {
            match nodes.len() {
                0 => return None,
                1 => return Some(nodes[0].clone()),
                _ => {
                    nodes.sort_by(|a, b| b.cmp(a));
                    let l: Node<u32> = nodes.pop().unwrap();
                    let r: Node<u32> = nodes.pop().unwrap();
                    nodes.push(Node::new_branch(l, r))
                }
            }
        }
    }
}
