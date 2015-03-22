use std::collections::HashMap;
use std::collections::hash_map::Entry::{Vacant, Occupied};
use std::fmt::{Debug, Formatter, Error};
use std::cmp::max;

pub type NodeIdx = usize;
pub type Variable = u16;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Node(pub Variable, pub NodeIdx, pub NodeIdx);

pub struct Forest {
    nodes: Vec<Node>,
    degrees: Vec<usize>,
    locations: HashMap<Node, NodeIdx>,
    pub sparsity: usize,
}

impl Forest {
    pub fn new() -> Forest {
        Forest {
            nodes: vec![Node(0, 0, 0), Node(0, 0, 0)],
            degrees: vec![0, 0],
            locations: HashMap::new(),
            sparsity: 100000,
        }
    }

    pub fn to_node(&self, idx: NodeIdx) -> Node {
        debug_assert!(idx > 1);

        self.nodes[idx]
    }

    pub fn degree(&self, idx: NodeIdx) -> usize { self.degrees[idx] }

    pub fn to_node_idx(&mut self, node: Node) -> NodeIdx {
        // If high idx is 0 or sparsity limit is hit,
        // remove node by returning low branch.
        if node.1 == 0 || self.degrees[node.1] + 1 == self.sparsity {
            return node.2
        }

        match self.locations.entry(node) {
            Vacant(e) => {
                let id = self.nodes.len();

                let hi_sparsity = self.degrees[node.1];
                let lo_sparsity = self.degrees[node.2];
                self.nodes.push(node);
                self.degrees.push(max(hi_sparsity + 1, lo_sparsity));
                e.insert(id);
                id
            },
            Occupied(e) => *e.get(),
        }
    }
}

impl Debug for Forest {
    fn fmt(&self, f :&mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.nodes.len())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn forest_basic() {
        let mut f = Forest::new();
        assert_eq!(f.nodes.len(), 2);

        f.to_node_idx(Node(0, 1, 0));
        assert_eq!(f.nodes.len(), 3);

        f.to_node_idx(Node(1, 1, 0));
        assert_eq!(f.nodes.len(), 4);

        f.to_node_idx(Node(2, 1, 0));
        assert_eq!(f.nodes.len(), 5);

        f.to_node_idx(Node(2, 0, 0));
        assert_eq!(f.nodes.len(), 5);

        f.to_node_idx(Node(2, 1, 0));
        assert_eq!(f.nodes.len(), 5);

        f.to_node_idx(Node(0, 1, 1));
        assert_eq!(f.nodes.len(), 6);
    }
}
