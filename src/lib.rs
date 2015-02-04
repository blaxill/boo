//#![deny(warnings)]
#![feature(core)]
#![feature(hash)]
#![feature(std_misc)]

pub use forest::{Forest, Node, NodeIdx};
pub use memoize::Memoize;
pub use add::add;
pub use multiply::multiply;
pub use degree::degree;
pub use lead::lead;
pub use divides::divides;
pub use monomial::is_monomial;
pub use divide::divide;
pub use spoly::spoly;

mod forest;
mod memoize;
mod add;
mod multiply;
mod degree;
mod lead;
mod divides;
mod monomial;
mod divide;
mod spoly;

pub struct Cache {
    add: Memoize<(NodeIdx, NodeIdx), NodeIdx>,
    multiply: Memoize<(NodeIdx, NodeIdx), NodeIdx>,
    lead: Memoize<NodeIdx, NodeIdx>,
    degree: Memoize<NodeIdx, NodeIdx>,
    divides: Memoize<(NodeIdx, NodeIdx), bool>,
    divide: Memoize<(NodeIdx, NodeIdx), NodeIdx>,
    spoly: Memoize<(NodeIdx, NodeIdx), NodeIdx>,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            add: Memoize::new(),
            multiply: Memoize::new(),
            lead: Memoize::new(),
            degree: Memoize::new(),
            divides: Memoize::new(),
            divide: Memoize::new(),
            spoly: Memoize::new(),
        }
    }
}

pub fn minmax<T: Ord>(lhs: T, rhs: T) -> (T, T) {
    if lhs > rhs { (rhs, lhs) }
    else { (lhs, rhs) }
}

//fn uid<T>(p: &T) -> usize { (p as * const _) as usize }
