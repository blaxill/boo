use super::node_hasher::NodeHasherState;

use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Error};
use std::cmp::max;
use std::collections::HashSet;
use std::default::Default;

pub type NodeIdx = usize;
pub type Variable = u8;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Node(pub Variable, pub NodeIdx, pub NodeIdx);

#[derive(Clone)]
struct NodePage {
    locations: HashMap<(NodeIdx, NodeIdx), NodeIdx, NodeHasherState>,
}

pub struct Forest {
    nodes: Vec<Node>,
    degrees: Vec<usize>,
    sparsity: usize,
    node_pages: Vec<NodePage>,
}

impl NodePage {
    fn new() -> NodePage {
        NodePage{ locations: HashMap::with_capacity_and_hash_state(1024*32, Default::default()), }
    }

    fn get_or_insert(&mut self, hi: NodeIdx, lo: NodeIdx, next_free: NodeIdx) -> NodeIdx {
        *self.locations.entry((hi, lo)).or_insert(next_free)
    }
}

impl Forest {
    pub fn new() -> Forest {
        Forest::with_sparsity(255)
    }

    pub fn with_sparsity(sparsity: usize) -> Forest {
        assert!(sparsity <= 256);
        Forest {
            nodes: vec![Node(0, 0, 0), Node(0, 0, 0)],
            degrees: vec![0, 0],
            sparsity: sparsity,
            node_pages: Vec::new(),
        }
    }

    pub fn to_node(&self, idx: NodeIdx) -> Node {
        debug_assert!(idx > 1);

        let high_bit = 0x8000_0000_0000_0000;
        let in_array = idx & high_bit > 0;

        if in_array { self.nodes[idx - high_bit] }
        else {
            let idx = idx;
            let var = (idx & ((1<<7)-1)) as u8 - 2;
            let lo = idx >> 7;

            Node(var, 1, lo)
        }
    }

    pub fn degree(&self, idx: NodeIdx) -> usize {
        let high_bit = 0x8000_0000_0000_0000;
        if idx < 2 { return 0 }
        if idx < high_bit { return 1 }
        self.degrees[idx - high_bit]
    }

    pub fn enforce_sparsity(&mut self, idx: NodeIdx, new_sparsity: usize)
        -> NodeIdx {
        if self.degree(idx) <= new_sparsity { return idx }

        if new_sparsity == 0 { return 0 }

        let Node(var, hi, lo) = self.to_node(idx);
        let hi = self.enforce_sparsity(hi, new_sparsity - 1);
        let lo = self.enforce_sparsity(lo, new_sparsity);

        if hi == 0 { return lo }

        self.to_node_idx(Node(var, hi, lo))
    }

    pub fn to_node_idx(&mut self, node: Node) -> NodeIdx {
        let sparsity = self.sparsity;
        let new_high = self.enforce_sparsity(node.1, sparsity - 1);
        let node = Node(node.0, new_high, node.2);

        // If high idx is 0,
        // remove node by returning low branch.
        if node.1 == 0 { return node.2 }

        let high_bit = 0x8000_0000_0000_0000;

        if node.0 < 64 && node.1 == 1 {
            if node.2 < (1<<55) {
                return (node.0 as usize) + 2 | (node.2 << 7);
            }
        }

        if node.0 as usize >= self.node_pages.len() {
            self.node_pages.resize(node.0 as usize + 1, NodePage::new());
        }

        let next_slot = self.nodes.len();
        let idx = self.node_pages[node.0 as usize].get_or_insert(node.1, node.2, next_slot);
        if idx == next_slot {
            let hi_sparsity = self.degree(node.1);// self.degrees[node.1];
            let lo_sparsity = self.degree(node.2);//self.degrees[node.2];
            self.nodes.push(node);
            self.degrees.push(max(hi_sparsity + 1, lo_sparsity));
        }
        idx | high_bit
    }

    pub fn evaluate(&self, idx: NodeIdx,
                    variable_map: &HashSet<Variable>) -> bool {
        if idx < 2 { return idx == 1 }

        let Node(var, hi, lo) = self.to_node(idx);
        let hi_eval = self.evaluate(hi, variable_map);
        let lo_eval = self.evaluate(lo, variable_map);

        if variable_map.contains(&var) {
            hi_eval ^ lo_eval
        } else { lo_eval }
    }

    pub fn sparsity(&self) -> usize { self.sparsity }
}

impl Debug for Forest {
    fn fmt(&self, f :&mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.nodes.len())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
    #[test]
    fn forest_basic() {
        let mut f = Forest::new();
        assert_eq!(f.nodes.len(), 2);

        f.to_node_idx(Node(1, 1, 0));
        assert_eq!(f.nodes.len(), 2);

        f.to_node_idx(Node(2, 1, 0));
        assert_eq!(f.nodes.len(), 3);

        f.to_node_idx(Node(2, 0, 0));
        assert_eq!(f.nodes.len(), 4);

        f.to_node_idx(Node(2, 1, 0));
        assert_eq!(f.nodes.len(), 4);
    }
    */
}
