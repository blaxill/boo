use std::cell::Cell;
use std::hash::{Hash, Hasher, Writer};
use ::NodeId;


pub struct Tree {
    target: Cell<NodeId>,
}

impl PartialEq for Tree {
    fn eq(&self, other: &Tree) -> bool {
        self as * const _  == other as * const _
    }
}

impl Eq for Tree {}

impl<H: Hasher + Writer> Hash<H> for Tree {
    fn hash(&self, state: &mut H) {
        (self as * const _).hash(state);
    }
}

/// Standalone methods to hide from public interface
pub fn new_tree(target: NodeId) -> Tree { Tree{ target: Cell::new(target)} }
pub fn get_target(this: &Tree) -> NodeId { this.target.get() }
pub fn set_target(this: &Tree, node: NodeId) { this.target.set(node) }
