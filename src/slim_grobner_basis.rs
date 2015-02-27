use super::forest::{Forest, NodeIdx};
use super::spoly::spoly;
use super::Cache;
use super::compare::compare;
use super::lead::lead;
use super::disjoint::disjoint_lead;
use super::minmax;
use super::reduce_basis::reduce_basis;
use super::divides::divides;
use super::degree::degree;

use std::collections::HashSet;
use std::iter::IntoIterator;
use std::cmp::{min, PartialEq, Ord, Ordering};

enum SlimReductionStrategy {
    LeadDivision(NodeIdx, NodeIdx),
    GroupReduction(NodeIdx),
    Replacement(NodeIdx, NodeIdx, NodeIdx),
}

impl SlimReductionStrategy {
    fn lead(&self) -> NodeIdx {
        match *self {
            SlimReductionStrategy::LeadDivision(_, m) => m,
            SlimReductionStrategy::GroupReduction(m) => m,
            SlimReductionStrategy::Replacement(_, _, m) => m,
        }
    }
}

impl PartialEq for SlimReductionStrategy {
    fn eq(&self, other: &SlimReductionStrategy) -> bool {
        self.lead() == other.lead()
    }
}

impl Eq for SlimReductionStrategy {}

fn slim_strategy_lead(c: &mut Cache,
                      f: &mut Forest,
                      rm: &Vec<(NodeIdx, NodeIdx)>,
                      gm: &Vec<(NodeIdx, NodeIdx)>) -> Option<SlimReductionStrategy> {
    for &rm in rm {
        for &gm in gm {
            if divides(c, f, gm.1, rm.1) {
                return Some(SlimReductionStrategy::LeadDivision(gm.0, rm.1));
            }
        }
    }

    None
}

fn slim_strategy_group(c: &mut Cache,
                       f: &mut Forest,
                       rm: &Vec<(NodeIdx, NodeIdx)>,
                       gm: &Vec<(NodeIdx, NodeIdx)>) -> Option<SlimReductionStrategy> {
    for &(_, m) in rm {
        let count = rm.iter()
            .fold(0, |acc, &(_, lead)|
                  acc + if m == lead { 1 } else { 0 }
                 );
        if count > 1 {
            return Some(SlimReductionStrategy::GroupReduction(m));
        }
    }

    None
}

fn slim_strategy_replace(c: &mut Cache,
                         f: &mut Forest,
                         rm: &Vec<(NodeIdx, NodeIdx)>,
                         gm: &Vec<(NodeIdx, NodeIdx)>) -> Option<SlimReductionStrategy> {
    for i in (0..rm.len()) {
        for j in (0..gm.len()) {
            if rm[i].1 == gm[j].1 &&
                degree(c, f, gm[j].1, None) > degree(c, f, rm[i].1, None) {
                return Some(SlimReductionStrategy::Replacement(gm[j].0, rm[j].0, gm[j].1));
            }
        }
    }
    None
}

fn slim_grobner_basis_reduce(c: &mut Cache,
                             f: &mut Forest,
                             r: Vec<NodeIdx>,
                             g: HashSet<NodeIdx>)
    -> (Vec<NodeIdx>, HashSet<NodeIdx>) {
    // Add leads as steps 1., 2. both will need them anyway.
    let mut rm: Vec<_> = r.iter().map(|&x| (x, lead(c, f, x, None)) ).collect();
    let mut gm: Vec<_> = g.iter().map(|&x| (x, lead(c, f, x, None)) ).collect();


    'outer: loop {
        rm.sort_by(|&(_, lhs), &(_, rhs)| compare(c, f, lhs, rhs));
        let mut choices: Vec<SlimReductionStrategy> = Vec::new();

        if let Some(x) = slim_strategy_replace(c, f, &rm, &gm) { choices.push(x); }
        if let Some(x) = slim_strategy_group(c, f, &rm, &gm) { choices.push(x); }
        if let Some(x) = slim_strategy_lead(c, f, &rm, &gm) { choices.push(x); }

        if choices.len() == 0 { break }
        choices.sort_by(|a, b| compare(c, f, a.lead(), b.lead()));

        match choices[0] {
            SlimReductionStrategy::GroupReduction(m) => {
                let queue: Vec<_> = rm.clone()
                    .into_iter()
                    .filter(|&(_, idx_lead)| idx_lead == m )
                    .collect();

                rm = rm .into_iter()
                    .filter(|&(_, idx_lead)| idx_lead != m )
                    .collect();

                for i in (0..queue.len()) {
                    for j in (i..queue.len()) {
                        let spoly = spoly(c, f, queue[i].0, queue[j].0);
                        if spoly != 0 { rm.push((spoly, lead(c, f, spoly, None))); }
                    }
                }

                rm.sort_by(|&(_, lhs), &(_, rhs)| compare(c, f, lhs, rhs));
                rm.dedup();
            },
            SlimReductionStrategy::LeadDivision(g, m) => {
                rm = rm.into_iter()
                    .filter_map(|(idx, idx_lead)| {
                        if idx_lead != m { return Some((idx, idx_lead)) }

                        let idx = spoly(c, f, idx, g);
                        if idx == 0 { return None }
                        let idx_lead = lead(c, f, idx, None);

                        Some((idx, idx_lead))
                    }).collect();

            },
            SlimReductionStrategy::Replacement(g, r, m) => {
                // TODO: replace pairs in parent function!!!
                for gm in &mut gm {
                    if gm.0 == g {
                        gm.0 = r;
                        gm.1 = m;
                    }
                }
                rm = rm.into_iter()
                    .filter_map(|(idx, idx_lead)| {
                        if idx != r { return Some((idx, idx_lead)) }

                        let idx = spoly(c, f, idx, g);
                        if idx == 0 { return None }
                        let idx_lead = lead(c, f, idx, None);

                        Some((idx, idx_lead))
                    }).collect();
            },
        }
    }

    (rm.into_iter().map(|(x, _)|x).collect(),
        gm.into_iter().map(|(x, _)|x).collect())
}

fn filter_pair(c: &mut Cache,
               f: &mut Forest,
               lhs: NodeIdx,
               rhs: NodeIdx) -> bool {
    let lhs_lead = lead(c, f, lhs, None);
    let rhs_lead = lead(c, f, rhs, None);

    disjoint_lead(c, f, lhs_lead, rhs_lead)
}

pub fn slim_grobner_basis<I>(c: &mut Cache,
                             f: &mut Forest,
                             polynomials: I,
                             max_reduce_set: usize,
                             max_iterations: Option<usize>) -> Vec<NodeIdx>
    where I: IntoIterator<Item = NodeIdx>,
{
    if max_reduce_set == 0 { panic!() }

    let mut max_iterations = max_iterations;

    let mut g: HashSet<NodeIdx> = HashSet::new();
    let mut p: Vec<(NodeIdx, NodeIdx)> = Vec::new();
    let polynomials: HashSet<NodeIdx> = polynomials.into_iter().collect();

    for poly in polynomials {
        for &g in &g {
            if filter_pair(c, f, g, poly) { continue }
            p.push(minmax(g, poly));
        }
        g.insert(poly);
    }

    while p.len() > 0 {
        let num = min(p.len(), max_reduce_set);

        let r: Vec<_> = p[..num].iter().filter_map(|&(lhs, rhs)|{
            let spoly = spoly(c, f, lhs, rhs);
            if spoly == 0 { None }
            else { Some(spoly) }
        }).collect();
        p = p[num..].to_vec();

        let (r, g_new) = slim_grobner_basis_reduce(c, f, r, g);
        g = g_new;

        for r in r {
            if g.contains(&r) { continue }

            for &g in &g {
                if filter_pair(c, f, g, r) { continue }
                p.push(minmax(g, r));
            }

            g.insert(r);
        }
        max_iterations = max_iterations.map(|x| x - 1);
        if let Some(x) = max_iterations { if x == 0 { break } }
    }

    g.into_iter().collect()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use super::super::forest::{Forest, Node, NodeIdx};
    use super::super::Cache;
    use super::super::add::add;
    use super::super::multiply::multiply;
    use super::super::enforce_sparsity::enforce_sparsity;
    use self::test::Bencher;

    #[test]
    fn slim_grobner_basis_basic() {
        let f = &mut Forest::new();
        let c = &mut Cache::new();

        let x = f.to_node_idx(Node(0, 1, 0));
        let y = f.to_node_idx(Node(1, 1, 0));
        let z = f.to_node_idx(Node(2, 1, 0));

        let z_add_y = add(c, f, z, y);
        let z_mul_x = multiply(c, f, z, x);
        let z_mul_x_add_y = add(c, f, z_mul_x, y);

        let v: Vec<NodeIdx> = vec![x, z_add_y, z_mul_x_add_y];

        println!("{:?}", v);
        println!("{:?}", slim_grobner_basis(c, f, v, 10, None));
    }


    fn build_polynomials(c: &mut Cache, f: &mut Forest) -> Vec<NodeIdx> {
        let i = 13;

        let mut v: Vec<_> = (0..i).map(|x| f.to_node_idx(Node(x, 1, 0))).collect();

        for _ in (0..3){
            for i in (0..v.len()) {
                if i % 3 == 0 { v[i] = add(c, f, v[i], 1); }
                else { v[i] = multiply(c, f, v[i], v[(i-1)%v.len()]) }
            }

            for i in (0..v.len()).rev() {
                for j in (i..v.len()) {
                    let n = f.to_node_idx(Node(j as u16, 1, 0));
                    v[i] = add(c, f, v[i], n);
                }
            }

            for i in (0..v.len()) {
                if i % 5 == 0 { v[i] = add(c, f, v[i], 1); }
                else { v[i] = multiply(c, f, v[i], v[(i-2)%v.len()]) }
            }
        }

        for i in (0..v.len()) {
            v[i] = enforce_sparsity(c, f, v[i], 6);
        }

        v
    }

    #[bench]
    fn bench_slim_grobner_basis_basic(b: &mut Bencher) {
        let f = &mut Forest::new();
        let c = &mut Cache::new();
        let v = build_polynomials(c, f);

        b.iter(|| {
            slim_grobner_basis(c, f, v.clone(), 35, Some(1_000_000_000))
        });
    }

    #[bench]
    fn z_bench_grobner_basis_basic(b: &mut Bencher) {
        let f = &mut Forest::new();
        let c = &mut Cache::new();
        let v = build_polynomials(c, f);

        b.iter(|| {
            slim_grobner_basis(c, f, v.clone(), 1, Some(1_000_000_000))
        });
    }
}
