use super::node::{Node, NodeIdx};
use super::forest::Forest;

pub fn add(f: &mut Forest,
           lhs: NodeIdx,
           rhs: NodeIdx) -> NodeIdx
{
    let (lhs, rhs) = if lhs < rhs { (lhs, rhs) } else { (rhs, lhs) };

    if lhs == 0 { return rhs }
    if lhs == rhs { return 0 }

    // At this point, lhs > 0, rhs > 1
    // e.g. rhs is not a terminal node
    match lhs {
        1 => {
            let Node(rhs_var, rhs_hi, rhs_lo) = f.to_node(rhs);
            let node = Node(rhs_var, rhs_hi, add(f, lhs, rhs_lo));
            f.to_node_idx(node)
        }
        _ => {
            let Node(lhs_var, lhs_hi, lhs_lo) = f.to_node(lhs);
            let Node(rhs_var, rhs_hi, rhs_lo) = f.to_node(rhs);
            let node = if lhs_var < rhs_var {
                    Node(lhs_var, lhs_hi, add(f, lhs_lo, rhs))
                } else if rhs_var < lhs_var {
                    Node(rhs_var, rhs_hi, add(f, rhs_lo, lhs))
                } else {
                    // lhs_var == rhs_var
                    Node(lhs_var, add(f, lhs_hi, rhs_hi),
                                  add(f, lhs_lo, rhs_lo))
                };

            f.to_node_idx(node)
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use super::super::node::Node;
    use super::super::forest::Forest;

    #[test]
    fn add_basic() {
        let f = &mut Forest::new();

        let x = f.to_node_idx(Node(0, 1, 0));
        let y = f.to_node_idx(Node(1, 1, 0));

        let x_add_y = add(f, x, y);
        let y_add_x = add(f, y, x);
        let x_x_y = add(f, x_add_y, x);

        assert_eq!(x_add_y, y_add_x);

        let Node(v, h, l) = f.to_node(x_add_y);

        assert_eq!(v, 0);
        assert_eq!(h, 1);
        assert_eq!(l, 3);

        let Node(v, h, l) = f.to_node(l);

        assert_eq!(v, 1);
        assert_eq!(h, 1);
        assert_eq!(l, 0);

        let Node(v, h, l) = f.to_node(x_x_y);

        assert_eq!(v, 1);
        assert_eq!(h, 1);
        assert_eq!(l, 0);

    }
}
