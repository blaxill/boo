use super::forest::{Forest, Node, NodeIdx};
use super::Cache;

pub fn disjoint(c: &mut Cache,
                f: &mut Forest,
                lhs: NodeIdx,
                rhs: NodeIdx) -> bool
{
    if lhs == rhs { return false }
    match (lhs, rhs) {
    }
    if lhs < 2 { return 0 }
    if rhs == 1 { return lhs }

    debug_assert!(rhs > 0);

    if let Some(result) = c.divide.get(&(lhs, rhs)) {
        return result
    }

    // At this point, lhs > 1, rhs > 1
    // e.g. neither are terminal nodes

    let Node(lhs_var, lhs_hi, lhs_lo) = f.to_node(lhs);
    let Node(rhs_var, rhs_hi, _) = f.to_node(rhs);

    let result = if lhs_var == rhs_var {
            divide(c, f, lhs_hi, rhs_hi)
        } else if rhs_var > lhs_var {
            let lhs_hi_div = divide(c, f, lhs_hi, rhs);
            let lhs_lo_div = divide(c, f, lhs_lo, rhs);
            f.to_node_idx(Node(lhs_var, lhs_hi_div, lhs_lo_div))
        } else { 0 };

    c.divide.set((lhs, rhs), result)
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::forest::{Forest, Node};
    use super::super::multiply::multiply;
    use super::super::add::add;
    use super::super::Cache;

    #[test]
    fn divide_basic() {
        let f = &mut Forest::new();
        let c = &mut Cache::new();

        let x = f.to_node_idx(Node(0, 1, 0));
        let y = f.to_node_idx(Node(1, 1, 0));

        let x_mul_y = multiply(c, f, x, y);
        let x_mul_y_add_1 = add(c, f, x_mul_y, 1);

        let x_add_1 = add(c, f, x, 1);

        assert_eq!(divide(c, f, 1, x), 0);
        assert_eq!(divide(c, f, x, x), 1);
        assert_eq!(divide(c, f, x_mul_y, x), y);
        assert_eq!(divide(c, f, x_mul_y_add_1, x), y);
        assert_eq!(divide(c, f, x_add_1, x), 1);
    }
}

