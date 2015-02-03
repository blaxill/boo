use std::collections::HashMap;
use std::collections::hash_map::Entry::{Vacant, Occupied};
use std::fmt::{Debug, Formatter, Error};

pub type NodeIdx = usize;
pub type Variable = u16;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Node {
    Node(Variable, NodeIdx, NodeIdx),
    True,
    False,
}

impl Node {
    pub fn unwrap(&self) -> (Variable, NodeIdx, NodeIdx) {
        match *self {
            Node::Node(v, h, l) => (v, h, l),
            _ => unreachable!()
        }
    }
}

pub struct Forest {
    nodes: Vec<Node>,
    locations: HashMap<Node, NodeIdx>,
}

impl Forest {
    pub fn new() -> Forest {
        Forest {
            nodes: vec![Node::False, Node::True],
            locations: {
                let mut locations = HashMap::new();
                locations.insert(Node::False, 0);
                locations.insert(Node::True, 1);
                locations
            },
        }
    }

    pub fn normalize(&self, node: Node) -> Node {
        match node {
            Node::Node(_, 0, l) => self.nodes[l],
            _ => node,
        }
    }

    pub fn to_node(&self, idx: NodeIdx) -> Node {
        self.nodes[idx]
    }

    pub fn to_node_idx(&mut self, node: Node) -> NodeIdx {
        let node = self.normalize(node);

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
    fn test_size() {
        let mut f = Forest::new();
        assert_eq!(f.nodes.len(), 2);

        f.to_node_idx(Node::Node(0, 1, 0));
        assert_eq!(f.nodes.len(), 3);

        f.to_node_idx(Node::Node(1, 1, 0));
        assert_eq!(f.nodes.len(), 4);

        f.to_node_idx(Node::Node(2, 1, 0));
        assert_eq!(f.nodes.len(), 5);

        f.to_node_idx(Node::Node(2, 0, 0));
        assert_eq!(f.nodes.len(), 5);

        f.to_node_idx(Node::Node(2, 1, 0));
        assert_eq!(f.nodes.len(), 5);

        f.to_node_idx(Node::Node(0, 1, 1));
        assert_eq!(f.nodes.len(), 6);
    }
}
