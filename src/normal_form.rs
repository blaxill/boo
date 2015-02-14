use super::forest::{Forest, Node, NodeIdx};
use super::Cache;
use super::divides::divides;
use super::divide::divide;
use super::lead::lead;
use super::compare::compare;
use super::multiply::multiply;
use super::add::add;
use super::terms_contains_term::terms_contains_term;

pub fn normal_form(
    c: &mut Cache,
    f: &mut Forest,
    reductee: NodeIdx,
    basis: Vec<NodeIdx>) -> NodeIdx
{
    if reductee == 0 { return 0 }

    let mut redux = reductee;
    let mut redux_lead = lead(c, f, redux, None);

    'outer: for x in basis {
        loop {
            if x == 0 { continue 'outer }
            let x_lead = lead(c, f, x, None);
            let mut terms = terms_contains_term(c, f, redux, x_lead);

            if terms.len() == 0 || terms[0] == 0 { continue 'outer }

            terms.sort_by(|&a, &b| compare(c, f, a, b));

            let highest = terms[0];

            redux = add(c, f, highest, redux);
            redux_lead = lead(c, f, redux, None);
        }
    }
    redux
}
