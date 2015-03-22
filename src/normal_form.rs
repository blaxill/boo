use super::forest::{Forest, NodeIdx};
use super::Cache;
use super::lead::lead;
use super::compare::compare;
use super::add::add;
use super::divide::divide;
use super::multiply::multiply;
use super::terms_contains_term::terms_contains_term;
use std::iter::IntoIterator;

use std::cmp::Ordering;

pub fn normal_form<'a, I>(
    c: &mut Cache,
    f: &mut Forest,
    reductee: NodeIdx,
    basis: I) -> NodeIdx
    where I: IntoIterator<Item = &'a NodeIdx>,
{
    if reductee == 0 { return 0 }

    let mut redux = reductee;

    'outer: for &x in basis {
        loop {
            if x == 0 { continue 'outer }
            let x_lead = lead(c, f, x, None);
            let mut terms = terms_contains_term(c, f, redux, x_lead);

            if terms.len() == 0 { continue 'outer }

            //terms.sort_by(|&a, &b| compare(c, f, a, b));
            //let t = terms[0];
            let t = terms.iter().fold(0, |v, &n| 
                              match compare(c, f, v, n){
                                  Ordering::Equal => v,
                                  Ordering::Less => v,
                                  Ordering::Greater => n,
                              });

            let m = divide(c, f, t, x_lead);
            assert!(m>0);
            let delta = multiply(c, f, x, m);

            redux = add(c, f, delta, redux);
        }
    }
    redux
}
