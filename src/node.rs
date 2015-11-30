pub type NodeIdx = usize;
pub type Variable = u8;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Node(pub Variable, pub NodeIdx, pub NodeIdx);
