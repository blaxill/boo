#![deny(warnings)]
#![feature(core)]

pub use memoize::Memoize;

pub mod memoize;

pub fn minmax<T: Ord>(lhs: T, rhs: T) -> (T, T) {
    if lhs > rhs { (rhs, lhs) }
    else { (lhs, rhs) }
}

#[cfg(test)]
mod test {
    use super::Memoize;

    #[test]
    fn memoize_print() {
        let x: Memoize<u8, u8> = Memoize::new();
        println!("{:?}", x);
    }
}

