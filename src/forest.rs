use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry::{Vacant, Occupied};
use std::iter::repeat;
use std::rc::{Rc, Weak, strong_count};
use std::fmt::{Debug, Formatter, Error};

pub type NodeId = usize;
pub type Term = u16;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Node {
    Variable(Term, NodeId, NodeId),
    True,
    False,
}

pub struct Forest {
    nodes: Vec<Node>,
    locations: HashMap<Node, NodeId>,

    // Memoization
    adds: HashMap<(NodeId, NodeId), NodeId>,
    muls: HashMap<(NodeId, NodeId), NodeId>,
    divs: HashMap<(NodeId, NodeId), NodeId>,
    cand: HashMap<(NodeId, NodeId), bool>,
    degr: HashMap<NodeId, NodeId>,
    lead: HashMap<NodeId, NodeId>,
    disj: HashMap<(NodeId, NodeId), bool>,
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

            adds: HashMap::new(),
            muls: HashMap::new(),
            divs: HashMap::new(),
            cand: HashMap::new(),
            degr: HashMap::new(),
            lead: HashMap::new(),
            disj: HashMap::new(),
        }
    }

    pub fn term(&mut self, t: Term) -> NodeId {
        self.get_node_id(Node::Variable(t, 1, 0))
    }

    fn follow_high(&self, node: NodeId) -> NodeId {
        match self.nodes[node] {
            Node::Variable(_, h, _) => h,
            _ => 0,
        }
    }

    fn follow_low(&self, node: NodeId) -> NodeId {
        match self.nodes[node] {
            Node::Variable(_, _, l) => l,
            _ => 0,
        }
    }

    pub fn get_node_id(&mut self, mut n: Node) -> NodeId {
        match n {
            Node::Variable(_, 0, l) => {
                n = self.nodes[l].clone();
            }
            _=>{},
        }
        match self.locations.entry(n) {
            Vacant(e) => {
                self.nodes.push(n);
                let id = self.nodes.len() - 1;
                e.insert(id);
                id
            },
            Occupied(e) => *e.get(),
        }
    }

    fn get_variable(&self, node: NodeId) -> Option<Term> {
        match self.nodes[node] {
            Node::Variable(v, _, _) => Some(v),
            _ => None,
        }
    }

    pub fn add_by_id(&mut self, mut lhs: NodeId, mut rhs: NodeId) -> NodeId {
        if lhs == 0 { return rhs }
        if rhs == 0 { return lhs }
        if lhs == rhs { return 0 }

        if lhs < rhs {
            let temp = lhs;
            lhs = rhs;
            rhs = temp;
        }

        if let Some(memoized) = self.adds.get(&(lhs, rhs)) {
            return *memoized;
        }

        let lo = self.get_variable(lhs);
        let ro = self.get_variable(rhs);

        let memoized = match (lo, ro) {
            (Some(lv), Some(rv)) => {
                if lv < rv {
                    let lo = self.follow_low(lhs);
                    let hi = self.follow_high(lhs);
                    let desc = Node::Variable(lv,
                           hi,
                           self.add_by_id(lo, rhs));
                    self.get_node_id(desc)
                } else if rv < lv {
                    let lo = self.follow_low(rhs);
                    let hi = self.follow_high(rhs);
                    let desc = Node::Variable(rv,
                           hi,
                           self.add_by_id(lo, lhs));
                    self.get_node_id(desc)
                } else {
                    let llo = self.follow_low(lhs);
                    let lhi = self.follow_high(lhs);
                    let rlo = self.follow_low(rhs);
                    let rhi = self.follow_high(rhs);

                    let desc = Node::Variable(lv,
                           self.add_by_id(lhi, rhi),
                           self.add_by_id(llo, rlo));
                    self.get_node_id(desc)
                }
            },
            (Some(lv), _) => {
                let lo = self.follow_low(lhs);
                let hi = self.follow_high(lhs);
                let desc = Node::Variable(lv, hi,
                           self.add_by_id(lo, rhs));
                self.get_node_id(desc)
            },
            (_, Some(rv)) => {
                let lo = self.follow_low(rhs);
                let hi = self.follow_high(rhs);
                let desc = Node::Variable(rv, hi,
                        self.add_by_id(lo, lhs));
                self.get_node_id(desc)
            },
            _ => panic!("Branch should never be reached"),
        };
        self.adds.insert((lhs, rhs), memoized);
        memoized
    }


    pub fn mul_by_id(&mut self, mut lhs: NodeId, mut rhs: NodeId) -> NodeId {
        if lhs == 1 { return rhs }
        if rhs == 1 { return lhs }
        if lhs == 0 || rhs == 0 { return 0 }
        if lhs == rhs { return lhs }

        if lhs < rhs {
            let temp = lhs;
            lhs = rhs;
            rhs = temp;
        }

        if let Some(memoized) = self.muls.get(&(lhs, rhs)) {
            return *memoized;
        }

        let lo = self.get_variable(lhs);
        let ro = self.get_variable(rhs);

        let (v, p1, p0, q1, q0) = match (lo, ro) {
            (Some(lv), Some(rv)) => {
                if lv < rv {
                    (lv,
                    self.follow_high(lhs),
                    self.follow_low(lhs),
                    0,
                    rhs)
                } else if rv < lv {
                    (rv,
                    self.follow_high(rhs),
                    self.follow_low(rhs),
                    0,
                    lhs)
                } else {
                    (lv,
                    self.follow_high(lhs),
                    self.follow_low(lhs),
                    self.follow_high(rhs),
                    self.follow_low(rhs))
                }
            },
            (Some(lv), _) => (lv,
                              self.follow_high(lhs),
                              self.follow_low(lhs),
                              0,
                              rhs),
            (_, Some(rv)) => (rv,
                              self.follow_high(rhs),
                              self.follow_low(rhs),
                              0,
                              lhs),
            _ => panic!("Branch should never be reached"),
        };

        let p0q0 = self.mul_by_id(p0, q0);
        let p0q1 = self.mul_by_id(p0, q1);
        let q0_q1 = self.add_by_id(q0, q1);
        let p1q0_p1q1 = self.mul_by_id(q0_q1, p1);
        let p0q1_p1q0_p1q1 = self.add_by_id(p0q1, p1q0_p1q1);

        let memoized = self.get_node_id(Node::Variable(v, p0q1_p1q0_p1q1, p0q0));
        self.muls.insert((lhs,rhs),memoized);
        memoized
    }

    pub fn lead_and_degree(&mut self, node: NodeId) -> (NodeId, usize) {
        if node < 2 { return (1, 0); }

        let hi = self.follow_high(node);
        let lo = self.follow_low(node);
        let (mut h1, d1) = self.lead_and_degree(hi);
        let (h0, d0) = self.lead_and_degree(lo);

        if d0 < d1 + 1 {
            //h1.insert(self.get_variable(node).unwrap());
            let v = self.get_variable(node).unwrap();
            h1 = self.get_node_id(Node::Variable(v, h1, 0));
            (h1, d1 + 1)
        } else {
            (h0, d0)
        }
    }

    pub fn cached_degree(&self, node: NodeId) -> Option<usize> {
        self.degr.get(&node).map(|&x|x)
    }

    pub fn degree(&mut self, node: NodeId) -> usize {
        if node < 2 { return 0 }
        if let Some(memoized) = self.degr.get(&node) {
            return *memoized;
        }

        let hi = self.follow_high(node);
        let lo = self.follow_low(node);
        let d1 = self.degree(hi) + 1;
        let d0 = self.degree(lo);

        let memoized = if d1 > d0 { d1 } else { d0 };

        self.degr.insert(node, memoized);
        memoized
    }

    pub fn degree_bounded(&mut self, node: NodeId, mut bound: usize) -> usize {
        if node < 2 || bound == 0 { return 0; }

        if let Some(memoized) = self.degr.get(&node) {
            if *memoized > bound { panic!("Bound contradiction!") }
            return *memoized;
        }

        let hi = self.follow_high(node);
        let lo = self.follow_low(node);
        let d1 = self.degree_bounded(hi, bound - 1) + 1;

        let memoized = if d1 == bound {
                d1
            } else {
                let d0 = self.degree_bounded(lo, bound);
                if d1 > d0 { d1 } else { d0 }
            };
        self.degr.insert(node, memoized);
        memoized
    }


    pub fn disjoint(&mut self, mut lhs: NodeId, mut rhs: NodeId) -> bool {
        if lhs < 2 || rhs < 2 { return true }

        if lhs > rhs {
            let temp = lhs;
            lhs = rhs;
            rhs = lhs;
        }

        if let Some(memoized) = self.disj.get(&(lhs, rhs)) {
            return *memoized;
        }

        let l_hi = self.follow_high(lhs);
        let r_hi = self.follow_high(rhs);

        let lv = self.get_variable(lhs);
        let rv = self.get_variable(rhs);

        let memoized = if lv == rv { false }
            else if lv < rv {
                self.disjoint(l_hi, rhs)
            } else {
                self.disjoint(lhs, r_hi)
            };

        self.disj.insert((lhs, rhs), memoized);
        memoized
    }

    pub fn lead(&mut self, node: NodeId) -> NodeId {
        if node < 2 { return node }

        if let Some(memoized) = self.lead.get(&node) {
            return *memoized;
        }

        let hi = self.follow_high(node);
        let d = self.degree(node);
        let d_hi = self.degree(hi);

        let memoized = if d == d_hi + 1 {
                let l = self.lead(hi);
                let v = self.get_variable(node).unwrap();
                self.get_node_id(Node::Variable(v, l, 0))
            } else {
                let lo = self.follow_low(node);
                self.lead(lo)
            };

        self.lead.insert(node, memoized);
        memoized
    }

    pub fn lead_bounded(&mut self, node: NodeId, mut bound: usize) -> NodeId {
        if node < 2 { return node }
        if bound == 0 { panic!("Bound not high enough for lead calc!") }

        if let Some(memoized) = self.lead.get(&node) {
            return *memoized;
        }

        let hi = self.follow_high(node);
        let d = self.degree_bounded(node, bound);
        let d_hi = self.degree_bounded(hi, bound - 1);

        let memoized = if d == d_hi + 1 {
                let l = self.lead_bounded(hi, bound - 1);
                let v = self.get_variable(node).unwrap();
                self.get_node_id(Node::Variable(v, l, 0))
            } else {
                let lo = self.follow_low(node);
                self.lead_bounded(lo, bound)
            };

        self.lead.insert(node, memoized);
        memoized
    }

    pub fn monomial_count(&self, node: NodeId) -> usize {
        match self.nodes[node] {
            Node::False => 0,
            Node::True => 1,
            Node::Variable(_, h, l) => {
                self.monomial_count(h) + self.monomial_count(l)
            }
        }
    }

    pub fn divides(&mut self, lhs_monomial: NodeId, rhs_monomial: NodeId) -> bool {
        debug_assert!(lhs_monomial > 0);
        debug_assert!(rhs_monomial > 0);

        if lhs_monomial == 1 { return true; }
        if rhs_monomial == 1 { return false; }

        if let Some(memoized) = self.cand.get(&(lhs_monomial, rhs_monomial)) {
            return *memoized;
        }

        let lo = self.get_variable(lhs_monomial);
        let ro = self.get_variable(rhs_monomial);

        let memoized = match (lo, ro) {
            (Some(lv), Some(rv)) => {
                if lv == rv {
                    let lhi = self.follow_high(lhs_monomial);
                    let rhi = self.follow_high(rhs_monomial);

                    self.divides(lhi, rhi)
                } else if rv < lv {
                    let hi = self.follow_high(rhs_monomial);

                    self.divides(lhs_monomial, hi)
                } else { false }
            }
            _ => panic!("Unreachable branch in divide!"),
        };
        self.cand.insert((lhs_monomial, rhs_monomial), memoized);
        memoized
    }

    pub fn divide_by_monomial(&mut self, poly: NodeId, monomial: NodeId) -> NodeId {
        if poly == monomial { return 1; }
        if poly < 2 { return 0; }
        if monomial == 1 { return poly; }
        if monomial == 0 { panic!("Divide by zero!") }

        if let Some(memoized) = self.divs.get(&(poly, monomial)) {
            return *memoized;
        }

        let lo = self.get_variable(poly);
        let ro = self.get_variable(monomial);

        let memoized = match (lo, ro) {
            (Some(lv), Some(rv)) => {
                if lv == rv {
                    let hi = self.follow_high(poly);
                    let hi_mono = self.follow_high(monomial);

                    self.divide_by_monomial(hi, hi_mono)
                } else if rv > lv {
                    let hi = self.follow_high(poly);
                    let lo = self.follow_low(poly);
                    let div_hi = self.divide_by_monomial(hi, monomial);
                    let div_lo = self.divide_by_monomial(lo, monomial);

                    //if div_hi == 0 { div_lo } else {
                        self.get_node_id(Node::Variable(lv, div_hi, div_lo))
                    //}
                } else { 0 }
            }
            _ => panic!("Unreachable branch in divide!"),
        };

        //if memoized > 1 { debug_assert!(self.follow_high(memoized) != 0); }

        self.divs.insert((poly,monomial),memoized);
        memoized
    }

    pub fn lcm(&mut self, lhs: NodeId, rhs: NodeId) -> NodeId {
        debug_assert!(lhs != 0);
        debug_assert!(rhs != 0);
        if lhs == 1 || rhs == 1 { return 1 }

        let lo = self.get_variable(lhs).unwrap();
        let ro = self.get_variable(rhs).unwrap();

        if lo < ro {
            let next = self.follow_high(lhs);
            self.lcm(next, rhs)
        } else if ro < lo {
            let next = self.follow_high(rhs);
            self.lcm(lhs, next)
        } else { // if lo == ro {
            let l_hi = self.follow_high(lhs);
            let r_hi = self.follow_high(rhs);
            let lcm = self.lcm(l_hi, r_hi);
            self.get_node_id(Node::Variable(lo, lcm, 0))
        }
    }

    pub fn is_term_equation(&self, node: NodeId) -> Option<(Term, bool)> {
        let hi = self.follow_high(node);
        let lo = self.follow_low(node);

        if hi > 1 || lo > 1 { return None }

        let v = self.get_variable(node).unwrap();

        Some((v, lo == 1))
    }

    pub fn evaluate(&self, node: NodeId, set_terms: &HashSet<Term>) -> bool {
        match node {
            0 => false,
            1 => true,
            n => {
                if let Some(v) = self.get_variable(n) {
                    self.evaluate(self.follow_low(n), set_terms) ^
                        if set_terms.contains(&v) {
                            self.evaluate(self.follow_high(n), set_terms)
                        } else {
                            false
                        }
                } else {
                    panic!("Branch in evaluation should never be reached!");
                }
            }
        }
    }
}

impl Debug for Forest {
    fn fmt(&self, f :&mut Formatter) -> Result<(), Error> {
        /*for (i, n) in self.nodes.iter().enumerate() {
            write!(f, "{}: ", i);
            n.fmt(f);
            writeln!(f, "");
        }*/
        writeln!(f, "adds: {}", self.adds.len());
        writeln!(f, "muls: {}", self.muls.len());
        writeln!(f, "divs: {}", self.divs.len());
        writeln!(f, "cand: {}", self.cand.len());
        writeln!(f, "degr: {}", self.degr.len());
        writeln!(f, "lead: {}", self.lead.len());
        writeln!(f, "disj: {}", self.disj.len());
        write!(f, "")
    }
}
