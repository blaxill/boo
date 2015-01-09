#![feature(slicing_syntax)]
#![allow(unstable)]

pub use tree::Tree;
pub use node::{NodeRef, NodeId};
pub use forest::{Forest, ForestContext};

mod tree;
mod node;
mod forest;

pub type Term = u16;

#[cfg(test)]
mod test{
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn basic() {
        let mut forest = Forest::new();

        forest.with(|f| {
            let _0 = f.constant(false);
            let _1 = f.constant(true);

            let _x = f.term(0);

            let res = f.add(_0, _1);
            let res2 = f.mul(_0, _1);

            println!("{}", f.evaluate(_1, &HashSet::new()));
            assert_eq!(res, _1);
            assert_eq!(res2, _0);
        });
    }
}
