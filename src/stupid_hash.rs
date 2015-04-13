use std::default::Default;
use std::hash::Hasher;
use std::collections::hash_state::HashState;

#[derive(Clone)]
pub struct NodeHasher {
    val: u64,
}

impl NodeHasher {
    #[inline]
    pub fn new() -> NodeHasher { NodeHasher{ val: 0 } }
}

impl Hasher for NodeHasher {
    #[inline]
    fn write(&mut self, msg: &[u8]) {
        for x in msg {
            self.val = self.val.wrapping_mul(18446744073709551557) ^ (*x as u64)
        }
    }
    #[inline]
    fn finish(&self) -> u64 {
        self.val
    }
}

#[derive(Clone)]
pub struct RandomState;

impl RandomState {
    #[inline]
    pub fn new() -> RandomState { RandomState }
}

impl HashState for RandomState {
    type Hasher = NodeHasher;

    #[inline]
    fn hasher(&self) -> NodeHasher {
        NodeHasher::new()
    }
}

impl Default for RandomState {
    #[inline]
    fn default() -> RandomState {
        RandomState::new()
    }
}
