use std::marker::InvariantLifetime;
use std::fmt::{Show, Formatter, Error};
use ::Term;

// Internal node type
#[derive(Show, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Node {
    Variable(Term, usize, usize),
    True,
    False,
}

/// Public node type for access during Forest.with()
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct NodeRef<'a> {
    id: NodeId,
    life: InvariantLifetime<'a>,
}

pub type NodeId = usize;

pub fn new_node_ref<'a>(id: NodeId) -> NodeRef<'a> {
    NodeRef{id: id, life: InvariantLifetime }
}

pub fn node_ref_to_id<'a>(this: NodeRef<'a>) -> NodeId {
    this.id
}

#[cfg(test)]
impl<'a> Show for NodeRef<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>{
        write!(f, "{}", self.id)
    }
}
