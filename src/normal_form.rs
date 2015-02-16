use super::forest::{Forest, NodeIdx};
use super::Cache;
use super::lead::lead;
use super::compare::compare;
use super::add::add;
use super::terms_containing::terms_containing;

pub fn normal_form(
    c: &mut Cache,
    f: &mut Forest,
    reductee: NodeIdx,
    basis: Vec<NodeIdx>) -> NodeIdx
{
    if reductee == 0 { return 0 }

    let mut redux = reductee;

    'outer: for x in basis {
        loop {
            if x == 0 { continue 'outer }
            let x_lead = lead(c, f, x, None);
            let mut terms = terms_containing(c, f, redux, x_lead);

            if terms.len() == 0 || terms[0] == 0 { continue 'outer }

            terms.sort_by(|&a, &b| compare(c, f, a, b));

            let highest = terms[0];

            redux = add(c, f, highest, redux);
        }
    }
    redux
}
