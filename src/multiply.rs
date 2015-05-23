use super::forest::{Forest, Node, NodeIdx};
use super::add::add;

pub fn multiply_with_sparsity(f: &mut Forest,
                              lhs: NodeIdx,
                              rhs: NodeIdx,
                              sparsity: usize) -> NodeIdx
{
    if sparsity == 0 { return 0 }

    let (lhs, rhs) = if lhs < rhs { (lhs, rhs) } else { (rhs, lhs) };

    if lhs == 0 { return 0 }
    if lhs == 1 { return rhs }
    if lhs == rhs { return lhs }

    let Node(lhs_var, lhs_hi, lhs_lo) = f.to_node(lhs);
    let Node(rhs_var, rhs_hi, rhs_lo) = f.to_node(rhs);

    let (v, p1, p0, q1, q0) = if lhs_var < rhs_var {
            (lhs_var, lhs_hi, lhs_lo, 0, rhs)
        } else if rhs_var < lhs_var {
            (rhs_var, rhs_hi, rhs_lo, 0, lhs)
        } else {
            (lhs_var, lhs_hi, lhs_lo, rhs_hi, rhs_lo)
        };

    let p0q0 = multiply_with_sparsity(f, p0, q0, sparsity);
    let p0q1 = multiply_with_sparsity(f, p0, q1, sparsity - 1);
    let q0_q1 = add(f, q0, q1);
    let p1q0_p1q1 = multiply_with_sparsity(f, q0_q1, p1, sparsity - 1);
    let p0q1_p1q0_p1q1 = add(f, p0q1, p1q0_p1q1);

    f.to_node_idx(Node(v, p0q1_p1q0_p1q1, p0q0))
}

pub fn multiply(f: &mut Forest,
                lhs: NodeIdx,
                rhs: NodeIdx) -> NodeIdx
{
    let sparsity = f.sparsity();
    multiply_with_sparsity(f, lhs, rhs, sparsity)
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::forest::{Forest, Node};

    #[test]
    fn multiply_basic() {
        let f = &mut Forest::new();

        let x = f.to_node_idx(Node(0, 1, 0));
        let y = f.to_node_idx(Node(1, 1, 0));

        let x_mul_y = multiply(f, x, y);
        let y_mul_x = multiply(f, y, x);

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

