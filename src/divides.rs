use super::forest::{Forest, Node, NodeIdx};
use super::monomial::is_monomial;
use super::Cache;

pub fn divides(c: &mut Cache,
               f: &mut Forest,
               lhs: NodeIdx,
               rhs: NodeIdx) -> bool
{
    debug_assert!(lhs > 0);
    //debug_assert!(rhs > 0);

    if cfg!(not(ndebug)) {
        assert!(is_monomial(f, lhs));
        assert!(is_monomial(f, rhs));
    }

    if lhs == 1 { return true }
    if rhs <= 1 { return false }

    if let Some(result) = c.divides.get(&(lhs, rhs)) {
        return result
    }

    let Node(lhs_var, lhs_hi, _) = f.to_node(lhs);
    let Node(rhs_var, rhs_hi, _) = f.to_node(rhs);

    let result = if lhs_var == rhs_var { divides(c, f, lhs_hi, rhs_hi) }
        else if rhs_var < lhs_var { divides(c, f, lhs, rhs_hi) }
        else { false };

    c.divides.set((lhs, rhs), result)
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::forest::{Forest, Node};
    use super::super::multiply::multiply;
    use super::super::Cache;

    #[test]
    fn divides_basic() {
        let f = &mut Forest::new();
        let c = &mut Cache::new();

        let x = f.to_node_idx(Node(0, 1, 0));
        let y = f.to_node_idx(Node(1, 1, 0));

        let x_mul_y = multiply(c, f, x, y);

        assert!(!divides(c, f, x, y));
        assert!(!divides(c, f, y, x));

        assert!(divides(c, f, 1, x));
        assert!(divides(c, f, x, x));
        assert!(divides(c, f, x, x_mul_y));
    }
}

