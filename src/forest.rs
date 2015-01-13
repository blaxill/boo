use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry::{Vacant, Occupied};
use std::iter::repeat;
use std::rc::{Rc, Weak, strong_count};

use ::Term;
use tree::{Tree, set_target, get_target, new_tree};
use node::{NodeRef, NodeId};
use node::{Node, new_node_ref, node_ref_to_id};


pub struct Forest {
    nodes: Vec<Node>,
    locations: HashMap<Node, NodeId>,

    trees: HashSet<Rc<Tree>>,
    tree_by_node: HashMap<NodeId, Weak<Tree>>,

    // Memoization
    adds: HashMap<(NodeId, NodeId), NodeId>,
    muls: HashMap<(NodeId, NodeId), NodeId>,
    divs: HashMap<(NodeId, NodeId), NodeId>,
}
pub struct ForestContext<'a> {
    parent: &'a mut Forest
}

impl<'a> ForestContext<'a> {
    pub fn constant(&self, t: bool) -> NodeRef<'a> {
        if t { new_node_ref(1) }
        else { new_node_ref(0) }
    }

    pub fn term(&mut self, v: Term) -> NodeRef<'a> {
        let id = self.parent.get_node_id(
                         &Node::Variable(v, 1, 0));
        new_node_ref(id)
    }

    pub fn add(&mut self, lhs: NodeRef, rhs: NodeRef)
        -> NodeRef<'a> {
        let id = self.parent.add_by_id(node_ref_to_id(lhs), node_ref_to_id(rhs));
        new_node_ref(id)
    }

    pub fn mul(&mut self, lhs: NodeRef, rhs: NodeRef)
        -> NodeRef<'a> {
        let id = self.parent.mul_by_id(node_ref_to_id(lhs), node_ref_to_id(rhs));
        new_node_ref(id)
    }

    pub fn evaluate(&self, node: NodeRef, set_terms: &HashSet<Term>) -> bool {
        self.parent.evaluate(node_ref_to_id(node), set_terms)
    }

    pub fn save_as_tree(&mut self, node: NodeRef) -> Rc<Tree> {
        self.parent.tree_from_node_id(node_ref_to_id(node))
    }

    pub fn tree_as_node(&mut self, tree: &Rc<Tree>) -> NodeRef<'a> {
        new_node_ref(get_target(&**tree))
    }
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
            trees: HashSet::new(),
            tree_by_node: HashMap::new(),

            adds: HashMap::new(),
            muls: HashMap::new(),
            divs: HashMap::new(),
        }
    }

    pub fn with<F>(&mut self, f: F)
        where F: FnOnce(&mut ForestContext) {
        let mut context = ForestContext {
            parent: self,
        };

        f(&mut context);
    }

    fn tree_from_node_id(&mut self, node: NodeId) -> Rc<Tree> {
        match self.tree_by_node.entry(node) {
            Vacant(e) => {
                let t = Rc::new(new_tree(node));
                self.trees.insert(t.clone());
                e.insert(t.clone().downgrade());
                t
            },
            Occupied(e) => {
                e.get().upgrade().unwrap()
            },
        }
    }

    pub fn new_empty_tree(&mut self) -> Rc<Tree> {
        self.tree_from_node_id(0)
    }

    pub fn release_unreferenced_nodes(&mut self) {
        let mut markers: Vec<bool> = repeat(false).take(self.nodes.len()).collect();

        markers[0] = true;
        markers[1] = true;

        self.trees = self.trees.iter()
            .filter(|&t| strong_count(t) > 1 )
            .map(|x|x.clone())
            .collect();

        for t in self.trees.iter(){
            markers[get_target(&**t)] = true;
        }

        for i in (2..self.nodes.len()).rev(){
            if markers[i] {
                let (h, l) = (self.follow_high(i), self.follow_low(i));
                if h > 1 { markers[h] = true; }
                if l > 1 { markers[l] = true; }
            }
        }

        let mapping: Vec<usize> = (0..self.nodes.len())
            .scan(0, |next_mapping, i| {
                if markers[i] {
                    *next_mapping += 1;
                    Some((*next_mapping)-1)
                } else {
                    Some(0)
                }
            }).collect();

        for t in self.trees.iter() {
            let previous = get_target(&**t);
            set_target(&**t, mapping[previous]);
        }

        self.tree_by_node = self.tree_by_node.iter()
            .filter_map(|(&node, t)| {
                match t.upgrade(){
                    Some(_) => {
                        Some((mapping[node], t.clone()))
                    }
                    None => None,
                }
            }).collect();

        self.nodes = self.nodes.iter()
            .zip(markers.iter())
            .filter_map(|(&n, &marker)|{
                if marker {
                    match n {
                        Node::Variable(t, h, l) => {
                            let h = mapping[h];
                            let l = mapping[l];
                            Some(Node::Variable(t, h, l))
                        }
                        _ => Some(n),
                    }
                } else { None }
            })
            .collect();

        self.adds = self.adds.iter()
            .filter_map(|(&(lhs, rhs), &res)|{
                if markers[lhs] && markers[rhs] && markers[res] {
                    Some( ((mapping[lhs], mapping[rhs]), mapping[res]) )
                } else {
                    None
                }
            })
            .collect();

        self.muls = self.muls.iter()
            .filter_map(|(&(lhs, rhs), &res)|{
                if markers[lhs] && markers[rhs] && markers[res] {
                    Some( ((mapping[lhs], mapping[rhs]), mapping[res]) )
                } else {
                    None
                }
            })
            .collect();

        self.locations = self.nodes.iter()
            .enumerate()
            .map(|(i, &n)|{
                (n, i)
            })
            .collect();
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

    fn get_node_id(&mut self, n: &Node) -> NodeId {
        match self.locations.entry(*n) {
            Vacant(e) => {
                self.nodes.push(*n);
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

    fn add_by_id(&mut self, mut lhs: NodeId, mut rhs: NodeId) -> NodeId {
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
                    self.get_node_id(&desc)
                } else if rv < lv {
                    let lo = self.follow_low(rhs);
                    let hi = self.follow_high(rhs);
                    let desc = Node::Variable(rv,
                           hi,
                           self.add_by_id(lo, lhs));
                    self.get_node_id(&desc)
                } else {
                    let llo = self.follow_low(lhs);
                    let lhi = self.follow_high(lhs);
                    let rlo = self.follow_low(rhs);
                    let rhi = self.follow_high(rhs);

                    let desc = Node::Variable(lv,
                           self.add_by_id(lhi, rhi),
                           self.add_by_id(llo, rlo));
                    self.get_node_id(&desc)
                }
            },
            (Some(lv), _) => {
                let lo = self.follow_low(lhs);
                let hi = self.follow_high(lhs);
                let desc = Node::Variable(lv, hi,
                           self.add_by_id(lo, rhs));
                self.get_node_id(&desc)
            },
            (_, Some(rv)) => {
                let lo = self.follow_low(rhs);
                let hi = self.follow_high(rhs);
                let desc = Node::Variable(rv, hi,
                        self.add_by_id(lo, lhs));
                self.get_node_id(&desc)
            },
            _ => panic!("Branch should never be reached"),
        };
        self.adds.insert((lhs, rhs), memoized);
        memoized
    }


    fn mul_by_id(&mut self, mut lhs: NodeId, mut rhs: NodeId) -> NodeId {
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

        let memoized = self.get_node_id(&Node::Variable(v, p0q1_p1q0_p1q1, p0q0));
        self.muls.insert((lhs,rhs),memoized);
        memoized
    }

    fn node_lead_and_degree(&mut self, node: NodeId) -> (NodeId, usize) {
        if node < 2 { return (1, 0); }

        let hi = self.follow_high(node);
        let lo = self.follow_low(node);
        let (mut h1, d1) = self.node_lead_and_degree(hi);
        let (h0, d0) = self.node_lead_and_degree(lo);

        if d0 < d1 + 1 {
            //h1.insert(self.get_variable(node).unwrap());
            let v = self.get_variable(node).unwrap();
            h1 = self.get_node_id(&Node::Variable(v, h1, 0));
            (h1, d1 + 1)
        } else {
            (h0, d0)
        }
    }

    fn node_monomial_count(&self, node: NodeId) -> usize {
        match self.nodes[node] {
            Node::False => 0,
            Node::True => 1,
            Node::Variable(_, h, l) => {
                self.node_monomial_count(h) + self.node_monomial_count(l)
            }
        }
    }

    fn node_divide_by_monomial(&mut self, poly: NodeId, monomial: NodeId) -> NodeId {
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
                let hi = self.follow_high(poly);
                let hi_mono = self.follow_high(monomial);
                if lv == rv {
                    self.node_divide_by_monomial(hi, hi_mono)
                } else if rv > lv {
                    let hi = self.follow_high(poly);
                    let lo = self.follow_low(poly);
                    let div_hi = self.node_divide_by_monomial(hi, monomial);
                    let div_lo = self.node_divide_by_monomial(lo, monomial);
                    self.get_node_id(&Node::Variable(lv, div_hi, div_lo))
                } else { 0 }
            }
            _ => panic!("Unreachable branch in divide!"),
        };

        self.divs.insert((poly,monomial),memoized);
        memoized
    }

    fn evaluate(&self, node: NodeId, set_terms: &HashSet<Term>) -> bool {
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


