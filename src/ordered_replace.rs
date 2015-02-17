use super::forest::{Forest, Node, NodeIdx, Variable};
use super::Cache;

fn ordered_replace_internal<F>(c: &mut Cache,
                      f: &mut Forest,
                      idx: NodeIdx,
                      func: &mut F) -> NodeIdx
    where F: FnMut(Variable) -> Variable
{
    if idx < 2 { return idx }

    let Node(var, hi, lo) = f.to_node(idx);

    let hi = ordered_replace_internal(c, f, hi, func);
    let lo = ordered_replace_internal(c, f, lo, func);

    let var = func(var);

    f.to_node_idx(Node(var, hi, lo))
}

pub fn ordered_replace<F>(c: &mut Cache,
                          f: &mut Forest,
                          idx: NodeIdx,
                          func: F) -> NodeIdx
    where F: FnMut(Variable) -> Variable
{
    let mut func = func;
    ordered_replace_internal(c, f, idx, &mut func)
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::forest::{Forest, Node};
    use super::super::Cache;
    use super::super::add::add;

    #[test]
    fn ordered_replace_basic() {
        let f = &mut Forest::new();
        let c = &mut Cache::new();

        let x = f.to_node_idx(Node(0, 1, 0));
        let y = f.to_node_idx(Node(1, 1, 0));
        let z = f.to_node_idx(Node(2, 1, 0));

        let x_add_y = add(c, f, x, y);
        let y_add_z = add(c, f, y, z);

        assert_eq!(y, ordered_replace(c, f, x, |v| v+1));
        assert_eq!(y, ordered_replace(c, f, x, |v| 2*v+1));
        assert_eq!(y_add_z, ordered_replace(c, f, x_add_y, |v| v+1));
    }
}
