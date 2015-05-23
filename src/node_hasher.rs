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
pub struct NodeHasherState;

impl NodeHasherState {
    #[inline]
    pub fn new() -> NodeHasherState { NodeHasherState }
}

impl HashState for NodeHasherState {
    type Hasher = NodeHasher;

    #[inline]
    fn hasher(&self) -> NodeHasher {
        NodeHasher::new()
    }
}

impl Default for NodeHasherState {
    #[inline]
    fn default() -> NodeHasherState {
        NodeHasherState::new()
    }
}
