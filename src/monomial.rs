use super::forest::{Forest, Node, NodeIdx};

pub fn is_monomial(f: &mut Forest,
                   idx: NodeIdx) -> bool
{
    if idx < 2 { return true }

    let Node(_, h, l) = f.to_node(idx);

    if l != 0 { return false }
    is_monomial(f, h)
}
