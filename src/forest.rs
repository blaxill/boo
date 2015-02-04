use std::collections::HashMap;
use std::collections::hash_map::Entry::{Vacant, Occupied};
use std::fmt::{Debug, Formatter, Error};

pub type NodeIdx = usize;
pub type Variable = u16;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Node(pub Variable, pub NodeIdx, pub NodeIdx);

pub struct Forest {
    nodes: Vec<Node>,
    locations: HashMap<Node, NodeIdx>,
}

impl Forest {
    pub fn new() -> Forest {
        Forest {
            nodes: vec![Node(0, 0, 0), Node(0, 0, 0)],
            locations: HashMap::new(),
        }
    }

    pub fn to_node(&self, idx: NodeIdx) -> Node {
        debug_assert!(idx > 1);

        self.nodes[idx]
    }

    pub fn to_node_idx(&mut self, node: Node) -> NodeIdx {
        // If high is 0, remove node by returning low branch.
        if node.1 == 0 { return node.2 }

        match self.locations.entry(node) {
            Vacant(e) => {
                let id = self.nodes.len();
                self.nodes.push(node);
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
