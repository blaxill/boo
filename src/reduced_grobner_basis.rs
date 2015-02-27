use super::forest::{Forest, NodeIdx};
use super::Cache;
use super::divides::divides;
use super::lead::lead;
use super::reduce_basis::reduce_basis;

pub fn reduced_grobner_basis(c: &mut Cache,
                             f: &mut Forest,
                             basis: Vec<NodeIdx>) -> Vec<NodeIdx> {
    let mut h: Vec<NodeIdx> = Vec::new();
    let mut basis: Vec<NodeIdx> = basis;

    'outer: while basis.len() > 0 {
        let f0 = basis.pop().unwrap();
        let f0_lead = lead(c, f, f0, None);

        for &b in &basis {
            let b_lead = lead(c, f, b, None);

            if divides(c, f, b_lead, f0_lead) {
                continue 'outer;
            }
        }
        for &h in &h {
            let h_lead = lead(c, f, h, None);

            if divides(c, f, h_lead, f0_lead) {
                continue 'outer;
            }
        }
        h.push(f0);
    }

    reduce_basis(c, f, h)
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::Cache;
    use super::super::slim_grobner_basis::slim_grobner_basis;
    use super::super::forest::{Forest, Node};
    use super::super::add::add;
    use super::super::multiply::multiply;

    #[test]
    fn reduce_grobner_basis_basic() {
        let f = &mut Forest::new();
        let c = &mut Cache::new();

        let x = f.to_node_idx(Node(0, 1, 0));
        let y = f.to_node_idx(Node(1, 1, 0));
        let z = f.to_node_idx(Node(2, 1, 0));

        let z_add_y = add(c, f, z, y);
        let z_mul_x = multiply(c, f, z, x);
        let z_mul_x_add_y = add(c, f, z_mul_x, y);

        let v = vec![x, z_add_y, z_mul_x_add_y];

        let slim = slim_grobner_basis(c, f, v.clone(), 1, None);
        let mut reduced = reduced_grobner_basis(c, f, slim);
        reduced.sort();
        assert_eq!(reduced, vec![x, y, z]);

        let slim = slim_grobner_basis(c, f, v.clone(), 5, None);
        let mut reduced = reduced_grobner_basis(c, f, slim);
        reduced.sort();
        assert_eq!(reduced, vec![x, y, z]);

        let slim = slim_grobner_basis(c, f, v.clone(), 50, None);
        let mut reduced = reduced_grobner_basis(c, f, slim);
        reduced.sort();
        assert_eq!(reduced, vec![x, y, z]);
    }
}
