use super::forest::{Forest, Node, NodeIdx};
use super::Cache;

pub fn term_count(c: &mut Cache,
                  f: &mut Forest,
                  idx: NodeIdx) -> NodeIdx
{
    if idx < 2 { return idx }

    let Node(_, hi, lo) = f.to_node(idx);

    return term_count(c, f, hi) + term_count(c, f, lo);
}
