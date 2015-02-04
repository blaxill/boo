use super::forest::{Forest, Node, NodeIdx};
use super::Cache;
use std::cmp::max;

pub fn degree(c: &mut Cache,
              f: &mut Forest,
              idx: NodeIdx,
              bound: Option<usize>) -> NodeIdx
{
    if idx < 2 { return 0 }

    if let Some(result) = c.degree.get(&idx) {
        return result
    }

    let Node(_, hi, lo) = f.to_node(idx);
    let hi_degree = degree(c, f, hi, bound.map(|x|x-1)) + 1;

    let result = match bound {
            Some(bound) if bound == hi_degree => hi_degree,
            _ => {
                let lo_degree = degree(c, f, lo, None);
                max(hi_degree, lo_degree)
            }
        };

    c.degree.set(idx, result)
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
