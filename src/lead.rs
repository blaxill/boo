use super::forest::{Forest, Node, NodeIdx};
use super::degree::degree;
use super::Cache;

pub fn lead(c: &mut Cache,
            f: &mut Forest,
            idx: NodeIdx,
            bound: Option<usize>) -> NodeIdx
{
    if idx < 2 { return idx }

    if let Some(result) = c.lead.get(&idx) {
        return result
    }

    let Node(var, hi, lo) = f.to_node(idx);
    let idx_degree = degree(c, f, idx, bound);
    let hi_degree = degree(c, f, hi, bound.map(|x|x-1));

    let result = if idx_degree == hi_degree + 1 {
            let hi_lead = lead(c, f, hi, bound.map(|x|x-1));
            f.to_node_idx(Node(var, hi_lead, 0))
        } else {
            lead(c, f, lo, bound)
        };

    c.lead.set(idx, result)
}


#[cfg(test)]
mod test {
    use super::*;
    use super::super::forest::{Forest, Node};
    use super::super::Cache;

    #[test]
    fn lead_basic() {
        let f = &mut Forest::new();
        let c = &mut Cache::new();

        let x = f.to_node_idx(Node(0, 1, 0));

        assert_eq!(x, lead(c, f, x, None));

        assert_eq!(1, lead(c, f, 1, None));
        assert_eq!(0, lead(c, f, 0, None));
    }
}
