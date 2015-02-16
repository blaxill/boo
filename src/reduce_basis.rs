use super::forest::{Forest, NodeIdx};
use super::Cache;
use super::normal_form::normal_form;
use std::collections::HashSet;

pub fn reduce_basis(
    c: &mut Cache,
    f: &mut Forest,
    basis: Vec<NodeIdx>) -> Vec<NodeIdx>
{
    let mut basis: HashSet<_> = basis.into_iter().collect();

    'restart: loop {
        let candidates: Vec<NodeIdx> = basis.iter().cloned().collect();

        for candidate_index in (0..candidates.len()) {
            let candidate: NodeIdx = candidates[candidate_index];
            basis.remove(&candidate);
            let reduced = normal_form(c, f, candidate, basis.iter().cloned().collect());
            if reduced == 0 { continue }
            basis.insert(reduced);
            if reduced != candidate { continue 'restart }
        }

        break;
    }

    basis.into_iter().collect()
}
