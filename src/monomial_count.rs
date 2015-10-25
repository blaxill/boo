use super::forest::{Forest, Node, NodeIdx};

pub fn monomial_count(f: &Forest,
                      idx: NodeIdx) -> usize
{
    if idx == 0 { return 0 }
    if idx == 1 { return 1}

    let Node(_, hi, lo) = f.to_node(idx);

    monomial_count(f, hi) + monomial_count(f, lo)
}

//TODO: add tests
