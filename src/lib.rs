#![feature(unboxed_closures)]
#![feature(hashmap_hasher)]

#![feature(test)]

extern crate test;

pub use node::{Node, Variable, NodeIdx};
pub use node_page::NodePage;
pub use forest::Forest;
pub use add::add;
pub use multiply::multiply;
pub use monomial_count::monomial_count;
pub use word::Word;
pub use node_hasher::NodeHasherState;

mod node;
mod node_page;
mod forest;
mod add;
mod multiply;
mod monomial_count;
mod word;
mod node_hasher;
