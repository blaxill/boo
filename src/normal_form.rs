use super::forest::{Forest, Node, NodeIdx};
use super::Cache;
use super::divides::divides;
use super::divide::divide;
use super::lead::lead;
use super::multiply::multiply;

pub fn normal_form(
    c: &mut Cache,
    f: &mut Forest,
    reductee: NodeIdx,
    basis: Vec<NodeIdx>) -> NodeIdx
{
    if reductee == 0 { return 0 }

    let mut redux = reductee;
    let mut redux_lead = lead(c, f, redux, None);

    for x in basis {
        if x == 0 { continue }
        let x_lead = lead(c, f, x, None);
        if !divides(c, f, x_lead, redux_lead) { continue }

        let remainder = divide(c, f, redux, x_lead);
        if remainder == 0 { continue }
        let m = multiply(c, f, remainder, x);
        let result = multiply(c, f, redux, m);

        if result == 0 { return 0 }

        redux = result;
        redux_lead = lead(c, f, redux, None);
    }
    redux
}
