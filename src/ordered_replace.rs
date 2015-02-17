use super::forest::{Forest, Node, NodeIdx, Variable};
use super::Cache;

pub fn ordered_replace<F>(c: &mut Cache,
                          f: &mut Forest,
                          idx: NodeIdx,
                          func: &mut F) -> NodeIdx
    where F: FnMut(Variable) -> Variable
{
    if idx < 2 { return idx }

    let Node(var, hi, lo) = f.to_node(idx);

    let hi = ordered_replace(c, f, hi, func);
    let lo = ordered_replace(c, f, lo, func);

    let var = func(var);

    f.to_node_idx(Node(var, hi, lo))
}


#[cfg(test)]
mod test {

    #[test]
    fn ordered_replace_basic() {
    }
}
