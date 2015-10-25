use std::collections::HashSet;
use std::hash::Hash;
use std::ops::BitXor;



pub trait Project {
    fn project(&self, sparsity: usize, exclusion_set: &HashSet<Self>) -> Self
        where Self: Sized;
}

pub struct CompressiveSensing<X, Y> {
    sparsity: usize,
    visited_points: HashSet<X>,
    target: Y,
    initial: X,
}

impl<X, Y> CompressiveSensing<X, Y>
    where X: BitXor<Output=X> + Eq + Hash + Clone + Project,
        Y: BitXor<Output=Y> + Clone + Eq {
    pub fn new(sparsity: usize, initial: X, target: Y) -> CompressiveSensing<X, Y> {
        CompressiveSensing {
            sparsity: sparsity,
            visited_points: HashSet::new(),
            target: target,
            initial: initial,
        }
    }

    pub fn compressive_sensing<T, U>(
        &mut self,
        transform: T,
        local_adjoint: U) -> X
        where T: Fn(X) -> Y,
              U: Fn(&X, Y) -> X {

        let mut x = self.initial.project(self.sparsity, &self.visited_points).clone();

        for _ in 0..10000 {
            let y = transform(x.clone());
            let y_delta = self.target.clone() ^ y;
            let x_delta = local_adjoint(&x, y_delta);
            x = (x ^ x_delta).project(self.sparsity, &self.visited_points);

            if transform(x.clone()) == self.target {
                return x
            }
        }

        x
    }
}

/*
// Note, this will not work as is.  Binary fields will likely have to be treated
// probablistically, using something like Maximum A Posteriori (MAP) estimation

//use super::word::Word;
impl<'a> Project for Word<'a> {
    fn project(&self,
               sparsity: usize,
               exclusion_set: &HashSet<Word<'a>>) -> Word<'a> {

        fn bitcount(mut value: u32) -> u32 {
            let mut output = 0;
            while value > 0 {
                if value & 1 > 0 {
                    output += 1;
                }
                value >>= 1;
            }
            output
        }

        let mask = self(&HashSet::new());

        for i in (0..mask).filter(|&x| bitcount(x) <= sparsity as u32).rev() {
            if (mask & i) == i {
                let candidate = Word::constant(self.forest, i);
                if !exclusion_set.contains(&candidate) {
                    return candidate;
                }
            }
        }

        Word::constant(self.forest, self(&HashSet::new()).wrapping_mul(132391).wrapping_add(256))
    }
}*/
