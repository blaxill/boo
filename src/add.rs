use super::forest::{Forest, Node, NodeIdx};
use super::{minmax, Cache};

pub fn add(c: &mut Cache,
           f: &mut Forest,
           lhs: NodeIdx,
           rhs: NodeIdx) -> NodeIdx
{
    let (lhs, rhs) = minmax(lhs, rhs);

    if lhs == 0 { return rhs }
    if lhs == rhs { return 0 }

    if let Some(result) = c.add.get(&(lhs, rhs)) {
        return result
    }

    // At this point, lhs > 0, rhs > 1
    // e.g. rhs is not a terminal node
    let result = match lhs {
        1 => {
            let Node(rhs_var, rhs_hi, rhs_lo) = f.to_node(rhs);
            let node = Node(rhs_var, rhs_hi, add(c, f, lhs, rhs_lo));
            f.to_node_idx(node)
        }
        _ => {
            let Node(lhs_var, lhs_hi, lhs_lo) = f.to_node(lhs);
            let Node(rhs_var, rhs_hi, rhs_lo) = f.to_node(rhs);
            let node = if lhs_var < rhs_var {
                    Node(lhs_var, lhs_hi, add(c, f, lhs_lo, rhs))
                } else if rhs_var < lhs_var {
                    Node(rhs_var, rhs_hi, add(c, f, rhs_lo, lhs))
                } else {
                    // lhs_var == rhs_var
                    Node(lhs_var, add(c, f, lhs_hi, rhs_hi),
                                  add(c, f, lhs_lo, rhs_lo))
                };

            f.to_node_idx(node)
        }
    };

    c.add.set((lhs, rhs), result)
}


#[cfg(test)]
mod test {
    use super::*;
    use super::super::forest::{Forest, Node};
    use super::super::Cache;

    #[test]
    fn add_basic() {
        let f = &mut Forest::new();
        let c = &mut Cache::new();

        let x = f.to_node_idx(Node(0, 1, 0));
        let y = f.to_node_idx(Node(1, 1, 0));

        let x_add_y = add(c, f, x, y);
        let y_add_x = add(c, f, y, x);

        assert_eq!(x_add_y, y_add_x);

        let Node(v, h, l) = f.to_node(x_add_y);

        assert_eq!(v, 0);
        assert_eq!(h, 1);
        assert_eq!(l, 3);

        let Node(v, h, l) = f.to_node(l);

        assert_eq!(v, 1);
        assert_eq!(h, 1);
        assert_eq!(l, 0);
    }
}
