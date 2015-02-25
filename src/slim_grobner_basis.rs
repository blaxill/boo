use super::forest::{Forest, NodeIdx};
use super::spoly::spoly;
use super::Cache;
use super::compare::compare;
use super::lead::lead;
use super::disjoint::disjoint;
use super::minmax;
use super::reduce_basis::reduce_basis;
use super::divides::divides;

use std::collections::HashSet;
use std::iter::IntoIterator;
use std::cmp::min;

fn slim_grobner_basis_reduce(c: &mut Cache,
                             f: &mut Forest,
                             r: Vec<NodeIdx>,
                             g: HashSet<NodeIdx>)
    -> (Vec<NodeIdx>, HashSet<NodeIdx>) {
    let mut rm: Vec<_> = r.iter().map(|&x| (x, lead(c, f, x, None)) ).collect();
    let mut gm: Vec<_> = g.iter().map(|&x| (x, lead(c, f, x, None)) ).collect();

    'outer: loop {
        let mut found: Option<(NodeIdx, NodeIdx)> = None;

        rm.sort_by(|&(_, lhs), &(_, rhs)| compare(c, f, lhs, rhs));

        // 2.

        for &(_, m) in &rm {
            let count = rm.iter()
                .fold(0, |acc, &(_, lead)|
                      acc + if m == lead { 1 } else { 0 }
                );
            if count > 1 {
                found = Some((m, 0));
                break;
            }
        }

        if let Some((m, _)) = found {
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

            continue 'outer;
        }

        // 1.

        'inner: for &rm in &rm {
            for &gm in &gm {
                if divides(c, f, gm.1, rm.1) {
                    found = Some((gm.0, rm.1));
                    break 'inner;
                }
            }
        }

        if let Some((g, m)) = found {
            rm = rm.into_iter()
                .filter_map(|(idx, idx_lead)| {
                    if idx_lead != m { return Some((idx, idx_lead)) }

                    let idx = spoly(c, f, idx, g);
                    if idx == 0 { return None }
                    let idx_lead = lead(c, f, idx, None);

                    Some((idx, idx_lead))
                }).collect();

            continue 'outer;
        }

        break;
    }

    (rm.into_iter().map(|(x, _)|x).collect(),
        gm.into_iter().map(|(x, _)|x).collect())
    //(r, g)
}

fn filter_p_criteria(c: &mut Cache,
                     f: &mut Forest,
                     p: &mut Vec<(usize, usize)>) {
    //p.sort_by(|&((_,_),a), &((_,_),b)| compare(c, f, a, b));
}

fn filter_pair(c: &mut Cache,
               f: &mut Forest,
               lhs: NodeIdx,
               rhs: NodeIdx) -> bool {
    let lhs_lead = lead(c, f, lhs, None);
    let rhs_lead = lead(c, f, rhs, None);

    disjoint(c, f, lhs, rhs)
}

pub fn slim_grobner_basis<I>(c: &mut Cache,
                             f: &mut Forest,
                             polynomials: I,
                             slim: bool) -> Vec<NodeIdx>
    where I: IntoIterator<Item = NodeIdx>,
{
    let mut max_iterations = 3_000_000;
    let mut max_reduce_set = 50;

    let mut g: HashSet<NodeIdx> = HashSet::new();
    let mut p: Vec<(NodeIdx, NodeIdx)> = Vec::new();
    let polynomials: HashSet<NodeIdx> = 
        reduce_basis(c, f, polynomials.into_iter().collect())
        .into_iter().collect();

    for poly in polynomials {
        for &g in &g {
            if filter_pair(c, f, g, poly) { continue }
            p.push(minmax(g, poly));
        }
        g.insert(poly);
    }

    if slim {
        while p.len() > 0 {
            let num = min(p.len(), max_reduce_set);

            let r: Vec<_> = p[..num].iter().map(|&(lhs, rhs)| spoly(c, f, lhs, rhs)).collect();
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
            max_iterations -= 1;
            if max_iterations == 0 { break }
        }
    } else {
        while p.len() > 0 {
            let (lhs, rhs) = p.pop().unwrap();

            let spoly = spoly(c, f, lhs, rhs);
            if spoly == 0 { continue }

            if g.contains(&spoly) { continue }

            for &g in &g {
                if filter_pair(c, f, g, spoly) { continue }
                p.push(minmax(g, spoly));
            }

            g.insert(spoly);

            max_iterations -= 1;
            if max_iterations == 0 { break }
        }
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
    use super::super::degree;
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
        println!("{:?}", slim_grobner_basis(c, f, v, true));
    }


    fn build_polynomials(c: &mut Cache, f: &mut Forest) -> Vec<NodeIdx> {
        use std::cmp::max;

        let i = 12;

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
            v[i] = enforce_sparsity(c, f, v[i], 5);
        }

        v
    }

    #[bench]
    fn bench_slim_grobner_basis_basic(b: &mut Bencher) {
        let f = &mut Forest::new();
        let c = &mut Cache::new();
        let mut v = build_polynomials(c, f);

        b.iter(|| {
            slim_grobner_basis(c, f, v.clone(), true)
        });
    }

    #[bench]
    fn z_bench_grobner_basis_basic(b: &mut Bencher) {
        let f = &mut Forest::new();
        let c = &mut Cache::new();
        let mut v = build_polynomials(c, f);

        b.iter(|| {
            slim_grobner_basis(c, f, v.clone(), false)
        });
    }
}
