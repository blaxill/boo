use super::forest::{Forest, Node, NodeIdx};
use super::Cache;

///TODO: Strip from codebase completely.
pub fn degree(c: &mut Cache,
              f: &mut Forest,
              idx: NodeIdx,
              bound: Option<usize>) -> NodeIdx
{
    return f.degree(idx);
}


#[cfg(test)]
mod test {
    use super::*;
    use super::super::forest::{Forest, Node};
    use super::super::Cache;

    #[test]
    fn degree_basic() {
        let f = &mut Forest::new();
        let c = &mut Cache::new();

        let x = f.to_node_idx(Node(0, 1, 0));

        assert_eq!(1, degree(c, f, x, None));
        assert_eq!(0, degree(c, f, 1, None));
        assert_eq!(0, degree(c, f, 0, None));
    }
}
