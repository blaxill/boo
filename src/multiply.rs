use super::forest::{Forest, Node, NodeIdx};
use super::add::add;
use super::{minmax, Cache};
use super::enforce_sparsity::enforce_sparsity;

pub fn multiply(c: &mut Cache,
                f: &mut Forest,
                lhs: NodeIdx,
                rhs: NodeIdx) -> NodeIdx
{
    let (lhs, rhs) = minmax(lhs, rhs);

    if lhs == 0 { return 0 }
    if lhs == 1 { return rhs }
    if lhs == rhs { return lhs }

    if let Some(result) = c.multiply.get(&(lhs, rhs)) {
        return result
    }

    // At this point, lhs > 1, rhs > 1
    // e.g. neither are terminal nodes

    let Node(lhs_var, lhs_hi, lhs_lo) = f.to_node(lhs);
    let Node(rhs_var, rhs_hi, rhs_lo) = f.to_node(rhs);

    let (v, p1, p0, q1, q0) = if lhs_var < rhs_var {
            (lhs_var, lhs_hi, lhs_lo, 0, rhs)
        } else if rhs_var < lhs_var {
            (rhs_var, rhs_hi, rhs_lo, 0, lhs)
        } else {
            (lhs_var, lhs_hi, lhs_lo, rhs_hi, rhs_lo)
        };

    let p0q0 = multiply(c, f, p0, q0);
    let p0q1 = multiply(c, f, p0, q1);
    let q0_q1 = add(c, f, q0, q1);
    let p1q0_p1q1 = multiply(c, f, q0_q1, p1);
    let p0q1_p1q0_p1q1 = add(c, f, p0q1, p1q0_p1q1);

    let result = f.to_node_idx(Node(v, p0q1_p1q0_p1q1, p0q0));
    let sparsity = f.sparsity;
    let result = enforce_sparsity(c, f, result, sparsity);

    c.multiply.set((lhs, rhs), result)
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::forest::{Forest, Node};
    use super::super::Cache;

    #[test]
    fn multiply_basic() {
        let f = &mut Forest::new();
        let c = &mut Cache::new();

        let x = f.to_node_idx(Node(0, 1, 0));
        let y = f.to_node_idx(Node(1, 1, 0));

        let x_mul_y = multiply(c, f, x, y);
        let y_mul_x = multiply(c, f, y, x);

        assert_eq!(x_mul_y, y_mul_x);

        let Node(v, h, l) = f.to_node(x_mul_y);

        assert_eq!(v, 0);
        assert_eq!(h, 3);
        assert_eq!(l, 0);

        let Node(v, h, l) = f.to_node(h);

        assert_eq!(v, 1);
        assert_eq!(h, 1);
        assert_eq!(l, 0);
    }
}

