use super::forest::{Forest, NodeIdx};
use super::Cache;

pub fn slim_grobner_basis(c: &mut Cache,
                          f: &mut Forest,
                          lhs: NodeIdx,
                          rhs: NodeIdx) -> NodeIdx
{
    0
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

    }
}
