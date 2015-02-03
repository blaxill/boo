use forest::{Forest, NodeId};
use std::collections::HashSet;
use std::iter::Chain;

pub fn spoly(f: &mut Forest, lhs: NodeId, rhs: NodeId) -> (NodeId, Option<usize>) {
    let lead_lhs = f.lead(lhs);
    let lead_rhs = f.lead(rhs);

    let prod = f.mul_by_id(lead_lhs, lead_rhs);
    let p_d_l = f.divide_by_monomial(prod, lead_lhs);
    let p_d_r = f.divide_by_monomial(prod, lead_rhs);

    let lhs_adj = f.mul_by_id(lhs, p_d_l);
    let rhs_adj = f.mul_by_id(rhs, p_d_r);

    let degree_hint = match (f.cached_degree(lhs_adj), f.cached_degree(rhs_adj)) {
        (Some(l_degr), Some(r_degr)) => if l_degr > r_degr {
            Some(l_degr)
        } else {
            Some(r_degr)
        },
        _ => None
    };

    (f.add_by_id(lhs_adj, rhs_adj), degree_hint)
}

pub fn greedy_normal_form<T: Iterator<Item=NodeId>>(
    f: &mut Forest, mut polys: T, reductee: NodeId, degree_hint: Option<usize>) -> NodeId {
    if reductee == 0 { return 0 }

    let mut redux = reductee;
    let mut redux_lead = if let Some(bound) = degree_hint {
            println!("Using hint!");
            f.lead_bounded(redux, bound)
        } else {
            f.lead(redux)
        };

    for x in polys {
        if x == 0 { continue }
        let lead = f.lead(x);
        if !f.divides(lead, redux_lead) { continue }

        let remainder = f.divide_by_monomial(redux, lead);
        if remainder == 0 { continue }
        let m = f.mul_by_id(remainder, x);
        let result = f.add_by_id(redux, m);

        if result == 0 { return 0 }

        let degree_hint = match (f.cached_degree(redux), f.cached_degree(m)) {
            (Some(l_degr), Some(r_degr)) => if l_degr > r_degr {
                Some(l_degr)
            } else {
                Some(r_degr)
            },
            _ => None
        };

        redux = result;
        redux_lead = if let Some(bound) = degree_hint {
            println!("Using hint!");
                f.lead_bounded(redux, bound)
            } else {
                f.lead(redux)
            };
    }
    redux
}

pub fn basis_reduction(f: &mut Forest, polys: HashSet<NodeId>) -> HashSet<NodeId> {
    let mut basis: HashSet<_> = polys;

    basis.remove(&0);

    'restart: loop {
        println!("Iterating in basis reduction, size: {}", basis.len());
        let candidates: Vec<NodeId> = basis.iter().cloned().collect();

        for candidate_index in (0..candidates.len()) {
            let candidate: NodeId = candidates[candidate_index];
            basis.remove(&candidate);
            let reduced = greedy_normal_form(f, basis.iter().cloned(), candidate, None);
            if reduced == 0 { continue }
            basis.insert(reduced);
            if reduced != candidate { continue 'restart }
        }

        break;
    }

    basis
}

pub fn grobner_test(f: &mut Forest, polys: HashSet<NodeId>) -> bool {
    let mut pairs: HashSet<(NodeId, NodeId)> = HashSet::with_capacity(polys.len()*polys.len());

    for &y in polys.iter(){
        let lead_y = f.lead(y);
        for &x in polys.iter() {
            if x == y { continue }
            let lead_x = f.lead(x);
            if f.disjoint(lead_x, lead_y) { continue }
            pairs.insert((x, y));
        }
    }

    while pairs.len() > 0 {
        let &(l, r) = pairs.iter().next().unwrap();
        let (h, degree_hint) = spoly(f, l, r);
        let h0 = greedy_normal_form(f, polys.iter().cloned(), h, degree_hint);
        if h0 == 0 { pairs.remove(&(l, r)); }
        else { return false }
    }


    return true;
}

pub fn grobner_basis(f: &mut Forest, polys: HashSet<NodeId>) -> HashSet<NodeId> {
    let mut pairs: Vec<(NodeId, NodeId, NodeId)> = Vec::with_capacity(polys.len()*polys.len());
    let mut basis: Vec<NodeId> = basis_reduction(f, polys).into_iter().collect();

    println!("Starting grobner basis routine");

    for &y in basis.iter(){
        let lead_y = f.lead(y);
        for &x in basis.iter() {
            if x == y { continue }
            let lead_x = f.lead(x);
            if f.disjoint(lead_x, lead_y) { continue }
            pairs.push((x, y, f.lcm(lead_x, lead_y)));
        }
    }

    while pairs.len() > 0 {
        pairs.sort_by(|a, b| f.cmp(a.2, b.2));
        println!("Pairs: {}\n{:?}", pairs.len(), f);
        let (l, r, lcm) = pairs.pop().unwrap();
        
        let (h, degree_hint) = spoly(f, l, r);
        let h0 = greedy_normal_form(f, basis.iter().cloned(), h, degree_hint);

        if h0 != 0 {
            let lead_h0 = f.lead(h0);
            for &g in basis.iter(){
                let lead_g = f.lead(g);
                if f.disjoint(lead_h0, lead_g) { continue }
                pairs.push((g, h0, f.lcm(lead_h0, lead_g)));
            }
            basis.push(h0);
        }
    }
    basis.into_iter().collect()
}

pub fn weighted_length(f: &Forest, p: NodeId) -> usize {
    f.support_count(p)
}

pub fn slimgb_strategy1(f: &mut Forest,
                        f_polys: Vec<NodeId>,
                        mut r_spolys: Vec<NodeId>) -> (Vec<NodeId>, Vec<NodeId>) {
    let r_lead: Vec<NodeId> = r_spolys.iter().map(|&x| f.lead(x)).collect();
    let mut r_sorted: Vec<NodeId> = r_lead.iter().cloned().collect();
    r_sorted.sort_by(|&a, &b| f.cmp(a,b));
    r_sorted.dedup();

    for i in (0..r_sorted.len()) {
        let m = r_sorted[i];
        let mut done = false;

        for &poly in f_polys.iter() {
            let f_lead = f.lead(poly);
            if f.divides(f_lead, m) {
                done = true;
                r_spolys = r_spolys.into_iter().map(|x| {
                    if f.lead(x) == m {
                        let (y,_ ) = spoly(f, x, poly);
                        y
                    } else { 
                        x
                    }
                }).collect();
            }
        }

        if done { break }
    }

    (f_polys, r_spolys)
}


