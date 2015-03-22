use super::forest::{Forest, Node, NodeIdx};
use super::Cache;
use super::lead::lead;
use super::degree::degree;

use std::cmp::Ordering;

pub fn reverse_compare(c: &mut Cache,
               f: &mut Forest,
               lhs: NodeIdx,
               rhs: NodeIdx) -> Ordering
{
    match compare(c, f, lhs, rhs) {
        Ordering::Equal => Ordering::Equal,
        Ordering::Greater => Ordering::Less,
        Ordering::Less => Ordering::Greater,
    }
}


pub fn compare(c: &mut Cache,
               f: &mut Forest,
               lhs: NodeIdx,
               rhs: NodeIdx) -> Ordering
{
    match (lhs, rhs) {
        (x, y) if x == y => return Ordering::Equal,
        (0, _) => return Ordering::Greater,
        (_, 0) => return Ordering::Less,
        (1, _) => return Ordering::Greater,
        (_, 1) => return Ordering::Less,
        _ => {},
    }

    let (lhs_degree, rhs_degree) = (degree(c, f, lhs, None), degree(c, f, rhs, None));

    if lhs_degree > rhs_degree { return Ordering::Less }
    if lhs_degree < rhs_degree { return Ordering::Greater }

    let (lhs_lead, rhs_lead) = (lead(c, f, lhs, None), lead(c, f, rhs, None));

    let Node(lhs_var, lhs_hi, _) = f.to_node(lhs_lead);
    let Node(rhs_var, rhs_hi, _) = f.to_node(rhs_lead);

    if lhs_var < rhs_var { Ordering::Less }
    else if rhs_var < lhs_var { Ordering::Greater }
    else { compare(c, f, lhs_hi, rhs_hi) }
}
