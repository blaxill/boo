//#![deny(warnings)]
#![feature(core)]
#![feature(test)]
#![feature(std_misc)]
#![feature(collections)]

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
pub use compare::compare;
pub use least_common_multiple::least_common_multiple;
pub use slim_grobner_basis::slim_grobner_basis;
pub use reduced_grobner_basis::reduced_grobner_basis;
pub use normal_form::normal_form;
pub use reduce_basis::reduce_basis;
pub use terms_contains_term::terms_contains_term;
pub use ordered_replace::ordered_replace;
pub use disjoint::disjoint_lead;
pub use enforce_sparsity::enforce_sparsity;
pub use term_count::term_count;
pub use word::Word;
pub use stupid_hash::RandomState;

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
mod compare;
mod least_common_multiple;
mod slim_grobner_basis;
mod reduced_grobner_basis;
mod normal_form;
mod reduce_basis;
mod terms_contains_term;
mod ordered_replace;
mod disjoint;
mod enforce_sparsity;
mod term_count;
mod word;
mod stupid_hash;

pub struct Cache {
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
}
