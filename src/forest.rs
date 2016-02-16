use super::node::*;
use super::node_page::NodePage;

use std::fmt::{Debug, Formatter, Error};
use std::cmp::max;
use std::collections::HashSet;
use std::io::{self, Write};

const HIGH_BIT: NodeIdx = 0x8000_0000_0000_0000;

pub struct Forest {
    nodes: Vec<Node>,
    degrees: Vec<usize>,
    sparsity: usize,
    node_pages: Vec<NodePage>,
}

impl Forest {
    pub fn new() -> Forest {
        Forest::with_sparsity(255)
    }

    pub fn write_graph<W: Write>(&self, writer: &mut W, idx: NodeIdx) -> io::Result<()> {
        if idx < 2 { return Ok(()); }

        let Node(var, hi, lo) = self.to_node(idx);


        match hi {
            1 => try!(writeln!(writer, "{} -> T", var)),
            0 => try!(writeln!(writer, "{} -> F", var)),
            x => {
                let Node(y, _, _) = self.to_node(x);
                try!(writeln!(writer, "{} -> {}", var, y));
            }
        }

        match lo {
            1 => try!(writeln!(writer, "{} -> T[style=\"dotted\"]", var)),
            0 => try!(writeln!(writer, "{} -> F[style=\"dotted\"]", var)),
            x => {
                let Node(y, _, _) = self.to_node(x);
                try!(writeln!(writer, "{} -> {}", var, y));
            }
        }
    
        try!(self.write_graph(writer, hi));
        try!(self.write_graph(writer, lo));

        return Ok(());
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

        let in_array = idx & HIGH_BIT > 0;

        if in_array {
            self.nodes[idx - HIGH_BIT]
        } else {
            let idx = idx;
            let var = (idx & ((1 << 7) - 1)) as u8 - 2;
            let lo = idx >> 7;

            Node(var, 1, lo)
        }
    }

    pub fn degree(&self, idx: NodeIdx) -> usize {
        if idx < 2 {
            return 0;
        }
        if idx < HIGH_BIT {
            return 1;
        }
        self.degrees[idx - HIGH_BIT]
    }

    pub fn enforce_sparsity(&mut self, idx: NodeIdx, new_sparsity: usize) -> NodeIdx {
        if self.degree(idx) <= new_sparsity {
            return idx;
        }

        if new_sparsity == 0 {
            return 0;
        }

        let Node(var, hi, lo) = self.to_node(idx);
        let hi = self.enforce_sparsity(hi, new_sparsity - 1);
        let lo = self.enforce_sparsity(lo, new_sparsity);

        if hi == 0 {
            return lo;
        }

        self.to_node_idx(Node(var, hi, lo))
    }

    pub fn to_node_idx(&mut self, node: Node) -> NodeIdx {
        let sparsity = self.sparsity;
        let new_high = self.enforce_sparsity(node.1, sparsity - 1);
        let node = Node(node.0, new_high, node.2);

        // If high idx is 0,
        // remove node by returning low branch.
        if node.1 == 0 {
            return node.2;
        }

        if node.0 < 64 && node.1 == 1 {
            if node.2 < (1 << 55) {
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
        idx | HIGH_BIT
    }

    pub fn evaluate(&self, idx: NodeIdx, variable_map: &HashSet<Variable>) -> bool {
        if idx < 2 {
            return idx == 1;
        }

        let Node(var, hi, lo) = self.to_node(idx);
        let hi_eval = self.evaluate(hi, variable_map);
        let lo_eval = self.evaluate(lo, variable_map);

        if variable_map.contains(&var) {
            hi_eval ^ lo_eval
        } else {
            lo_eval
        }
    }

    pub fn sparsity(&self) -> usize {
        self.sparsity
    }
}

impl Debug for Forest {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.nodes.len())
    }
}

#[cfg(test)]
mod test {
    //
    // use super::*;
    //
    // #[test]
    // fn forest_basic() {
    // let mut f = Forest::new();
    // assert_eq!(f.nodes.len(), 2);
    //
    // f.to_node_idx(Node(1, 1, 0));
    // assert_eq!(f.nodes.len(), 2);
    //
    // f.to_node_idx(Node(2, 1, 0));
    // assert_eq!(f.nodes.len(), 3);
    //
    // f.to_node_idx(Node(2, 0, 0));
    // assert_eq!(f.nodes.len(), 4);
    //
    // f.to_node_idx(Node(2, 1, 0));
    // assert_eq!(f.nodes.len(), 4);
    // }
    //
}
