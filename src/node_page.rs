use super::node::NodeIdx;

const HIGH_BIT: NodeIdx = 0x8000_0000_0000_0000;
const DEFAULT_SIZE: usize = 256;
const MAX_DELTA: usize = 8;

#[derive(Clone)]
pub struct NodePage {
    // idx, hi, lo
    locations: Vec<(NodeIdx, NodeIdx, NodeIdx)>,
    size: usize,
}

impl NodePage {
    pub fn new() -> NodePage {
        NodePage {
            locations: (0..DEFAULT_SIZE).map(|_| (HIGH_BIT, 0, 0)).collect(),
            size: DEFAULT_SIZE,
        }
    }

    pub fn get_or_insert(&mut self, hi: NodeIdx, lo: NodeIdx, next_free: NodeIdx) -> NodeIdx {
        if next_free >= HIGH_BIT {
            panic!("Overflowing into high bit on next NodeIdx!");
        }

        let hash = (hi ^ (lo << 32) ^ (lo >> 32)) & (self.size - 1);
        let mut delta = 0;

        loop {
            if delta > MAX_DELTA {
                self.size <<= 1;
                self.locations.resize(self.size, (HIGH_BIT, 0, 0));
                return self.get_or_insert(hi, lo, next_free);
            }

            let loc = hash.wrapping_add(delta) & (self.size - 1);
            let (idx, x, y) = self.locations[loc];

            if idx >= HIGH_BIT {
                self.locations[loc] = (next_free, hi, lo);
                return next_free;
            }

            if x == hi && lo == y {
                return idx;
            }

            delta += 1;
        }

        // *self.locations.entry((hi, lo)).or_insert(next_free)
    }
}
