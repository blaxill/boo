use super::forest::{Forest, Node, NodeIdx};
use super::Cache;

/// all monomials of lhs containing rhs
pub fn terms_containing(c: &mut Cache,
                        f: &mut Forest,
                        lhs: NodeIdx,
                        rhs: NodeIdx) -> Vec<NodeIdx>
{
    match (lhs, rhs) {
        (0, _) => return Vec::new(),
        (1, 0) => return vec![1],
        (1, 1) => return vec![1],
        (1, _) => return Vec::new(),
        _ => {},
    }

    let Node(lhs_var, lhs_hi, lhs_lo) = f.to_node(lhs);
    let Node(rhs_var, rhs_hi, _) = f.to_node(rhs);

    if rhs_var < lhs_var {
        Vec::new()
    } else if rhs_var == lhs_var {
        let mut terms_hi = terms_containing(c, f, lhs_hi, rhs_hi).map_in_place(
            |t| f.to_node_idx(Node(lhs_var, t, 0))
            );
        terms_hi
    } else {
        let mut terms_hi = terms_containing(c, f, lhs_hi, rhs).map_in_place(
            |t| f.to_node_idx(Node(lhs_var, t, 0))
            );
        let mut terms_lo = terms_containing(c, f, lhs_lo, rhs);

        terms_hi.append(&mut terms_lo);
        terms_hi
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use super::super::forest::{Forest, Node};
    use super::super::Cache;
    use super::super::add::add;
    use super::super::multiply::multiply;

    #[test]
    fn terms_containing_basic() {
        let f = &mut Forest::new();
        let c = &mut Cache::new();

        let x = f.to_node_idx(Node(0, 1, 0));
        let y = f.to_node_idx(Node(1, 1, 0));

        let x_add_y = add(c, f, x, y);
        let x_mul_y = multiply(c, f, x, y);

        assert_eq!(terms_containing(c, f, x_add_y, x).len(), 1);
        assert_eq!(terms_containing(c, f, x_add_y, x)[0], x);
        assert_eq!(terms_containing(c, f, x_add_y, y).len(), 1);
        assert_eq!(terms_containing(c, f, x_add_y, y)[0], y);

        assert_eq!(terms_containing(c, f, x_mul_y, y).len(), 1);
        assert_eq!(terms_containing(c, f, x_mul_y, y)[0], x_mul_y);
    }
}
