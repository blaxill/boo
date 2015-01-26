use forest::{Forest, NodeId};
use std::collections::HashSet;
use std::iter::Chain;

fn reduce(f: &mut Forest,
          o: NodeId,
          e: NodeId) -> Option<NodeId>{
    let (lead, _) = f.lead_and_degree(e);
    let remainder = f.divide_by_monomial(o, lead);
    let m = f.mul_by_id(remainder, e);
    let result = f.add_by_id(o, m);

    if m == 0 || result == 0 { return None }

    Some(result)
}

pub fn normal_form(f: &mut Forest,
                   equations: Vec<NodeId>) -> Vec<NodeId> {
    let mut reductors: HashSet<NodeId> = equations.into_iter().collect();
    let mut count = reductors.len();

    loop {
        reductors = reductors
            .iter()
            .flat_map(|ref x|
                reductors
                    .iter()
                    .map(|ref y| **x)
                    )
            .collect();

        let last_count = count;
        count = reductors.len();
        if last_count == count { break; }
    }
    reductors.into_iter().collect()
}
