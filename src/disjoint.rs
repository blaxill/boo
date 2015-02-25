use super::forest::{Forest, Node, NodeIdx};
use super::Cache;

pub fn disjoint(c: &mut Cache,
                f: &mut Forest,
                lhs: NodeIdx,
                rhs: NodeIdx) -> bool
{
    if lhs == rhs { return false }
    //if lhs < 2 || rhs < 2 { return true }
    match (lhs, rhs) {
        (1, _) => return false,
        (0, _) => return true,
        (_, 1) => return false,
        (_, 0) => return true,
        _ => {},
    }

    let Node(lhs_var, lhs_hi, lhs_lo) = f.to_node(lhs);
    let Node(rhs_var, rhs_hi, rhs_lo) = f.to_node(rhs);

    if lhs_var < rhs_var {
        true //disjoint(c, f, lhs_hi, rhs)
    } else if rhs_var < lhs_var {
        true //disjoint(c, f, lhs, rhs_hi)
    } else { disjoint(c, f, lhs_hi, rhs_hi) }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::forest::{Forest, Node};
    use super::super::multiply::multiply;
    use super::super::add::add;
    use super::super::Cache;

    #[test]
    fn disjoint_basic() {
        let f = &mut Forest::new();
        let c = &mut Cache::new();

        let x = f.to_node_idx(Node(0, 1, 0));
        let y = f.to_node_idx(Node(1, 1, 0));
        let z = f.to_node_idx(Node(2, 1, 0));

        let x_mul_y = multiply(c, f, x, y);

        assert_eq!(disjoint(c, f, 1, x), false);
        assert_eq!(disjoint(c, f, 1, y), false);
        assert_eq!(disjoint(c, f, x_mul_y, x), false);
        assert_eq!(disjoint(c, f, x, x), false);
        assert_eq!(disjoint(c, f, x, y), true);
        assert_eq!(disjoint(c, f, x_mul_y, z), true);
    }
}

