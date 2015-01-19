use forest::{Forest, NodeId};
use std::collections::HashSet;

pub fn normal_form(f: &mut Forest,
              equations: Vec<NodeId>) -> Vec<NodeId> {
    let mut reductors: HashSet<NodeId> = equations.into_iter().collect();
    let mut results = reductors.clone();

    loop {
        for &e in reductors.iter() {
            let reduced = reductors.iter()
                .filter_map(|&o| {
                    let (lead, _) = f.lead_and_degree(e);
                    let remainder = f.divide_by_monomial(o, lead);
                    let m = f.mul_by_id(remainder, e);
                    let result = f.add_by_id(o, m);
                    /*println!("{} = {} + {}*{} (={})",
                             result, o, remainder, e, m);*/

                    if m == 0 || result == 0 { return None }

                    Some(result)
                });
            results = results.into_iter().chain(reduced).collect();
        }
        if results.len() == reductors.len() { break; }
        reductors = results.clone();
    }
    results.into_iter().collect()
}
