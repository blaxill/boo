use super::forest::{Forest, NodeIdx};
use super::spoly::spoly;
use super::Cache;
use super::compare::compare;

use std::collections::HashSet;
use std::iter::IntoIterator;

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
            p.push((g, poly));
        }
        g.insert(poly);
    }

    filter_p_criteria(c, f, &mut p);

    while p.len() > 0 {
        let (lhs, rhs) = p.pop().unwrap();

        let spoly = spoly(c, f, lhs, rhs);
        if spoly == 0 { continue }

        if g.contains(&spoly) { continue }

        for &g in &g {
            p.push((g, spoly));
        }

        g.insert(spoly);
    }

    g.into_iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::forest::{Forest, Node};
    use super::super::Cache;
    use super::super::add::add;
    use super::super::multiply::multiply;

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
}
