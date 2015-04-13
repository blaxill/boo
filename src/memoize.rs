use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::{Debug, Formatter, Error};
use std::default::Default;
use std::collections::hash_state::HashState;

/// TODO: LRU-type cache, or random dropout for unrecalled items
pub struct Memoize<I: Hash + Eq, O, S>
    (HashMap<I, O, S>);

impl<I, O, S> Memoize<I, O, S>
    where I: Hash + Eq,
          O: Copy,
          S: HashState + Default,
{
    pub fn new() -> Memoize<I, O, S> { Memoize(HashMap::with_hash_state(Default::default())) }

    pub fn get(&self, input: &I) -> Option<O> { self.0.get(input).map(|&x| x) }

    pub fn set(&mut self, input: I, output: O) -> O {
        self.0.insert(input, output);
        output
    }
}

impl<I, O, S> Debug for Memoize<I, O, S>
    where I: Hash + Eq,
          S: HashState,
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0.len())
    }
}

