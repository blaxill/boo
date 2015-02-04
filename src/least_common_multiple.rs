use super::forest::{Forest, Node, NodeIdx};
use super::{minmax, Cache};

pub fn least_common_multiple(c: &mut Cache,
              f: &mut Forest,
              lhs: NodeIdx,
              rhs: NodeIdx) -> NodeIdx
{
    debug_assert!(lhs != 0);
    debug_assert!(rhs != 0);
    if lhs == 1 || rhs == 1 { return 1 }

    let (lhs, rhs) = minmax(lhs, rhs);

    if let Some(result) = c.least_common_multiple.get(&(lhs, rhs)) {
        return result
    }

    // At this point, lhs > 1, rhs > 1
    // e.g. neither are terminal nodes

    let Node(lhs_var, lhs_hi, _) = f.to_node(lhs);
    let Node(rhs_var, rhs_hi, _) = f.to_node(rhs);

    let result = if lhs_var == rhs_var {
            let lcm = least_common_multiple(c, f, lhs_hi, rhs_hi);
            f.to_node_idx(Node(lhs_var, lcm, 0))
        } else if rhs_var > lhs_var {
            least_common_multiple(c, f, lhs_hi, rhs)
        } else {
            least_common_multiple(c, f, lhs, rhs_hi)
        };

    c.least_common_multiple.set((lhs, rhs), result)
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::forest::{Forest, Node};
    use super::super::multiply::multiply;
    use super::super::Cache;

    #[test]
    fn lcm_basic() {
        let f = &mut Forest::new();
        let c = &mut Cache::new();

        let x = f.to_node_idx(Node(0, 1, 0));
        let y = f.to_node_idx(Node(1, 1, 0));

        let x_mul_y = multiply(c, f, x, y);

        assert_eq!(least_common_multiple(c, f, 1, x), 1);
        assert_eq!(least_common_multiple(c, f, x, x), x);
        assert_eq!(least_common_multiple(c, f, x_mul_y, x), x);
    }
}

