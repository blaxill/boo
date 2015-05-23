use std::collections::HashSet;
use std::hash::Hash;
use std::ops::BitXor;

pub trait Project {
    fn project(&self, sparsity: usize, exclusion_set: &HashSet<Self>) -> Self;
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

        for _ in (0..10000) {
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
