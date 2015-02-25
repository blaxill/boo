use super::forest::{Forest, NodeIdx};
use super::spoly::spoly;
use super::Cache;
use super::compare::compare;
use super::lead::lead;

use std::collections::HashSet;
use std::iter::IntoIterator;

extern crate test;

fn slim_grobner_basis_reduce(c: &mut Cache,
                             f: &mut Forest,
                             s: Vec<(usize, usize)>,
                             g: Vec<NodeIdx>)
    -> (Vec<NodeIdx>, Vec<NodeIdx>) {
    let mut r: Vec<NodeIdx> = Vec::new();

    for s in s {
        r.push(spoly(c, f, s.0, s.1));
    }

    (r, g)
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
    return false;
}

pub fn slim_grobner_basis<I, T>(c: &mut Cache,
                                f: &mut Forest,
                                polynomials: I) -> Vec<NodeIdx>
    where I: IntoIterator<IntoIter = T>,
          T: Iterator<Item = NodeIdx>
{
    let mut g: HashSet<NodeIdx> = HashSet::new();
//    g.sort_by(|&a, &b| compare(c, f, a, b));
 //   g.dedup();
    let mut p: Vec<(NodeIdx, NodeIdx)> = Vec::new();

    for poly in polynomials {
        for &g in &g {
            if filter_pair(c, f, g, poly) { continue }
            p.push((g, poly));
        }
        g.insert(poly);
    }

    while p.len() > 0 {
        let (lhs, rhs) = p.pop().unwrap();

        let spoly = spoly(c, f, lhs, rhs);
        if spoly == 0 { continue }

        if g.contains(&spoly) { continue }

        for &g in &g {
            if filter_pair(c, f, g, spoly) { continue }
            p.push((g, spoly));
        }

        g.insert(spoly);
    }

    g.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::forest::{Forest, Node};
    use super::super::Cache;
    use super::super::add::add;
    use super::super::multiply::multiply;
    use super::test::Bencher;
    use super::super::degree;

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

        let v = vec![x, z_add_y, z_mul_x_add_y];

        println!("{:?}", v);
        println!("{:?}", slim_grobner_basis(c, f, v));

    }

    #[bench]
    fn bench_slim_grobner_basis_basic(b: &mut Bencher) {
        use std::cmp::max;

        let f = &mut Forest::new();
        let c = &mut Cache::new();

        let mut v: Vec<_> = (0..6).map(|x| f.to_node_idx(Node(x, 1, 0))).collect();

        for _ in (0..1) {
            for i in (0..v.len()) {
                if i % 3 == 0 { v[i] = add(c, f, v[i], 1); }
                else { v[i] = multiply(c, f, v[i], v[(i-1)%v.len()]) }
            }

            for i in (0..v.len()) {
                if i % 5 == 0 { v[i] = add(c, f, v[i], 1); }
                else { v[i] = multiply(c, f, v[i], v[(i-2)%v.len()]) }
            }

            for i in (0..v.len()) {
                let z = f.to_node_idx(Node(((i-3)%v.len()) as u16, 1, 0));
                v[i] = multiply(c, f, v[i], z);
            }
        }

        println!("Max degree dense: {}", v
                 .iter()
                 .fold(0, |acc, &v| 
                       max(acc, degree(c, f, v, None) )
                   ));

        b.iter(|| {
            slim_grobner_basis(c, f, v.clone())
        });
    }

    #[bench]
    fn bench_sparse_slim_grobner_basis_basic(b: &mut Bencher) {
        use std::cmp::max;
        use super::super::enforce_sparsity::enforce_sparsity;

        let f = &mut Forest::new();
        let c = &mut Cache::new();

        let mut v: Vec<_> = (0..6).map(|x| f.to_node_idx(Node(x, 1, 0))).collect();

        for _ in (0..1) {
            for i in (0..v.len()) {
                if i % 3 == 0 { v[i] = add(c, f, v[i], 1); }
                else { v[i] = multiply(c, f, v[i], v[(i-1)%v.len()]) }
            }

            for i in (0..v.len()) {
                if i % 5 == 0 { v[i] = add(c, f, v[i], 1); }
                else { v[i] = multiply(c, f, v[i], v[(i-2)%v.len()]) }
            }

            for i in (0..v.len()) {
                let z = f.to_node_idx(Node(((i-3)%v.len()) as u16, 1, 0));
                v[i] = multiply(c, f, v[i], z);
            }
        }

        for i in (0..v.len()) {
            v[i] = enforce_sparsity(c, f, v[i], 2);
        }

        println!("Max degree sparse: {}", v
                 .iter()
                 .fold(0, |acc, &v| 
                       max(acc, degree(c, f, v, None) )
                ));

        b.iter(|| {
            slim_grobner_basis(c, f, v.clone())
        });
    }
}
