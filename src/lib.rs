//#![deny(warnings)]
#![feature(core)]
#![feature(hash)]
#![feature(std_misc)]

pub use forest::Forest;
pub use memoize::Memoize;
pub use add::add;

mod forest;
mod memoize;
mod add;

pub fn minmax<T: Ord>(lhs: T, rhs: T) -> (T, T) {
    if lhs > rhs { (rhs, lhs) }
    else { (rhs, lhs) }
}

fn uid<T>(p: &T) -> usize { (p as * const _) as usize }
