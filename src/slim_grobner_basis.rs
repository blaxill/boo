use super::forest::{Forest, Node, NodeIdx};
use super::spoly::spoly;
use super::Cache;
use super::compare::compare;

use std::iter::IntoIterator;
use std::cmp::min;

fn slim_grobner_basis_reduce(c: &mut Cache,
                             forest: &mut Forest,
                             s: Vec<((usize, usize), NodeIdx)>,
                             f: Vec<NodeIdx>)
    -> (Vec<NodeIdx>, Vec<NodeIdx>) {
    //(Vec::new(), Vec::new())
    let mut r: Vec<NodeIdx> = Vec::new();

    for s in s {
        r.push(s.1);
    }

    (r, f)
}

fn filter_p_criteria(c: &mut Cache,
                     f: &mut Forest,
                     p: &mut Vec<((usize, usize), NodeIdx)>) {
    p.sort_by(|&((_,_),a), &((_,_),b)| compare(c, f, a, b));
}

pub fn slim_grobner_basis<I, T>(c: &mut Cache,
                                forest: &mut Forest,
                                polynomials: I) -> Vec<NodeIdx>
    where I: IntoIterator<Iter = T>,
          T: Iterator<Item = NodeIdx>
{
    let mut f: Vec<NodeIdx> = polynomials.into_iter().collect();
    f.sort_by(|&a, &b| compare(c, forest, a, b));
    f.dedup();
    let mut p: Vec<((usize, usize), NodeIdx)> = Vec::new();

    for (i, &x) in f.iter().enumerate() {
        for (j, &y) in f.iter().enumerate() {
            if x == y { continue }

            let spoly = spoly(c, forest, x, y);
            if spoly == 0 { continue }

            p.push(((i, j), spoly));
        }
    }

    filter_p_criteria(c, forest, &mut p);

    while p.len() > 0 {
        println!("p: {:?}", p);
        println!("f: {:?}", f);
        let p_len = p.len();
        //let s = p.split_off(min(p_len-1, 4));
        let mut s = p;

        if s.len() > 3 {
            p = s.split_off(3);
        } else {
            p = Vec::new();
        }

        let (mut r, mut next_f) = slim_grobner_basis_reduce(c, forest, s, f);

        for &r in &r {
            if r == 0 { continue }
            if let Some(_) = next_f.iter().position(|&x| x==r) { continue }

            let r_idx = next_f.len();
            for i in (0..next_f.len()){
                let spoly = spoly(c, forest, next_f[i], r);
                if spoly == 0 { continue }
                p.push(((i, r_idx), spoly));
            }
            next_f.push(r);
        }

        f = next_f;
        filter_p_criteria(c, forest, &mut p);
    }

    f
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

        let v = vec![x, z_add_y, z_mul_x];

        println!("{:?}", v);
        println!("{:?}", slim_grobner_basis(c, f, v));
    }
}
