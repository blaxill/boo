use forest::{Forest, NodeId};
use std::collections::HashSet;
use std::iter::Chain;

pub fn normal_form(f: &mut Forest,
                   equations: Vec<NodeId>) -> Vec<NodeId> {
    let mut reduced: HashSet<_> = equations.into_iter().collect();
    let mut k = 1;
    loop {
        let mut out = HashSet::with_capacity(reduced.len());
        /*for (i, &x) in reduced.iter().enumerate() {
            println!("Iter grobner b. step {} of {}", i+1, reduced.len());
            let lead = f.lead(x);

            for &y in reduced.iter() {
                if x == y { continue }

                let remainder = f.divide_by_monomial(y, lead);
                let m = f.mul_by_id(remainder, x);
                let result = f.add_by_id(y, m);

                if m == 0 || result == 0 { continue }

                out.insert(result);
            }
        }*/
        'outer: for (i, &y) in reduced.iter().enumerate() {
            println!("Iter {} grobner b. step {} of {}", k, i+1, reduced.len());
            println!("---------------");
            println!("{:?}", f);

            let mut redux = y;

            for (j, &x) in reduced.iter().enumerate() {
                let lead = f.lead(x);
                if i == j { continue }

                let remainder = f.divide_by_monomial(redux, lead);
                if remainder == 0 { continue }
                let m = f.mul_by_id(remainder, x);
                let result = f.add_by_id(redux, m);

                if result == 0 { continue 'outer }

                redux = result;
            }

            out.insert(redux);
        }

        k=k+1;

        if out == reduced { break; } //out.len() == 0 { break }
        //for e in out.into_iter() { reduced.insert(e); }
        reduced = out;
    }
    reduced.into_iter().collect()
}
