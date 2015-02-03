use std::collections::HashMap;
use std::collections::hash_map::Hasher;
use std::hash::Hash;
use std::fmt::{Debug, Formatter, Error};

/// Intrusive memoizer.
/// TODO: LRU-type cache, or random dropout for unrecalled items
pub struct Memoize<I: Hash<Hasher> + Eq, O>
    (HashMap<I, O>);

impl<I, O> Memoize<I, O>
    where I: Hash<Hasher> + Eq,
          O: Copy
{
    pub fn new() -> Memoize<I, O> { Memoize(HashMap::new()) }

    pub fn get(&self, input: &I) -> Option<O> { self.0.get(input).map(|&x| x) }

    pub fn set(&mut self, input: I, output: O) -> O {
        self.0.insert(input, output);
        output
    }
}

impl<I, O> Debug for Memoize<I, O>
    where I: Hash<Hasher> + Eq
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0.len())
    }
}

