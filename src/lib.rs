#![feature(core)]
#![feature(hash)]
#![feature(test)]
#![feature(std_misc)]
#![feature(collections)]
#![feature(unboxed_closures)]
#![feature(append)]
#![feature(vec_resize)]
#![feature(hashmap_hasher)]
#![feature(iter_min_max)]

extern crate test;

pub use forest::{Forest, Node, NodeIdx};
pub use add::add;
pub use multiply::multiply;
pub use monomial_count::monomial_count;
pub use word::Word;
pub use node_hasher::NodeHasherState;
pub use compressive_sensing::CompressiveSensing;

//pub use memoize::Memoize;

mod forest;
mod add;
mod multiply;
mod monomial_count;
mod word;
mod node_hasher;
mod compressive_sensing;

//mod memoize;

pub trait CachingStrategy<I, O> {
    fn get(&/*mut*/self, input: &I) -> Option<&O>;
    fn set(&/*mut*/self, input: I, output: O);
}

/*pub struct Cache {
    add: Memoize<(NodeIdx, NodeIdx), NodeIdx, RandomState>,
    multiply: Memoize<(NodeIdx, NodeIdx), (NodeIdx, usize), RandomState>,
    lead: Memoize<NodeIdx, NodeIdx, RandomState>,
    degree: Memoize<NodeIdx, NodeIdx, RandomState>,
    divides: Memoize<(NodeIdx, NodeIdx), bool, RandomState>,
    divide: Memoize<(NodeIdx, NodeIdx), NodeIdx, RandomState>,
    spoly: Memoize<(NodeIdx, NodeIdx), NodeIdx, RandomState>,
    least_common_multiple: Memoize<(NodeIdx, NodeIdx), NodeIdx, RandomState>,
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
            least_common_multiple: Memoize::new(),
        }
    }
}

pub fn minmax<T: Ord>(lhs: T, rhs: T) -> (T, T) {
    if lhs > rhs { (rhs, lhs) }
    else { (lhs, rhs) }
}*/
