use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::{Debug, Formatter, Error};
use std::default::Default;
use std::collections::hash_state::HashState;

/// TODO: LRU-type cache, or random dropout for unrecalled items
pub struct Memoize<I: Hash + Eq, O, S> {
    map: HashMap<I, O, S>,
    counts: HashMap<I, usize, S>,
}

impl<I, O, S> Memoize<I, O, S>
    where I: Hash + Eq + Clone,
          O: Copy,
          S: HashState + Default,
{
    pub fn new() -> Memoize<I, O, S> { 
        Memoize{
            map: HashMap::with_hash_state(Default::default()),
            counts: HashMap::with_hash_state(Default::default()),
        }
    }

    pub fn get(&mut self, input: &I) -> Option<O> {
        let next_count = if let Some(count) = self.counts.get(input) {
                count + 1
            } else {
                0
            };
        self.counts.insert((*input).clone(), next_count);
        self.map.get(input).map(|&x| x)
    }

    pub fn set(&mut self, input: I, output: O) -> O {
        self.map.insert(input, output);
        output
    }
}

impl<I, O, S> Debug for Memoize<I, O, S>
    where I: Hash + Eq,
          S: HashState,
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let min_max = self.counts.iter().map(|(_, &count)| count).min_max();
        let avg = self.counts.iter()
            .map(|(_, &count)| count)
            .fold(0f64, |avg, count| avg + count as f64) / self.counts.len() as f64;
        writeln!(f, "Memoize {{ length: {}, min/max hits: {:?} }}", self.map.len(), min_max)
    }
}

