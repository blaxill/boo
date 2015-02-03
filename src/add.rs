use super::forest::{Forest, Node, NodeIdx};
use super::memoize::Memoize;
use super::{minmax, uid};
use std::cell::RefCell;

thread_local!(static MEMOIZED_ADD:
              RefCell<Memoize<(usize, NodeIdx, NodeIdx), NodeIdx>>
              = RefCell::new(Memoize::new()) );


pub fn add(f: &mut Forest,
           lhs: NodeIdx,
           rhs: NodeIdx) -> NodeIdx
{
    let (lhs, rhs) = minmax(lhs, rhs);

    if lhs == 0 { return rhs }
    if lhs == rhs { return 0 }

    if let Some(result) = MEMOIZED_ADD.with(|c| {
        c.borrow().get(&(uid(f), lhs, rhs))
    }) {
        return result
    }

    // At this point, lhs > 0, rhs > 1
    // e.g. rhs is not a terminal node
    let result = match lhs {
        1 => {
            let (rhs_var, rhs_hi, rhs_lo) = f.to_node(lhs).unwrap();
            let node = Node::Node(rhs_var, rhs_hi, add(f, lhs, rhs_lo));
            f.to_node_idx(node)
        }
        _ => {
            let (lhs_var, lhs_hi, lhs_lo) = f.to_node(lhs).unwrap();
            let (rhs_var, rhs_hi, rhs_lo) = f.to_node(lhs).unwrap();
            let node = if lhs_var < rhs_var {
                    Node::Node(lhs_var, lhs_hi, add(f, lhs_lo, rhs))
                } else if rhs_var < lhs_var {
                    Node::Node(rhs_var, rhs_hi, add(f, rhs_lo, lhs))
                } else { // lhs_var == rhs_var
                    Node::Node(lhs_var, add(f, lhs_hi, rhs_hi),
                                        add(f, lhs_lo, rhs_lo))
                };
            f.to_node_idx(node)
        }
    };

    MEMOIZED_ADD.with(|c| c.borrow_mut().set((uid(f), lhs, rhs), result) )
}


