use std::cmp::PartialOrd;
use num::Unsigned;

/// This struct represents a node in a linked list implementation of a priority
/// queue. The V type represents the type of value stored in the node, and the
/// P type represents the kind of unsigned value that is being used to denote
/// the priority of the node.
pub struct PqNode<V, P>
    where V: PartialOrd, P: Unsigned,
{
    pub value:V,
    pub priority:P,
    // pub next:Option<&'a PqNode<'a, V, P>>,
    pub next:Option<Box<PqNode<V, P>>>,
}

impl<'a, V, P> PqNode<V, P>
    where V: PartialOrd, P: Unsigned,
{
    pub fn new(value:V, priority:P, next:Option<Box<PqNode<V, P>>>) -> Self {
        PqNode { value, priority, next }
    }

    pub fn length(&self) -> u32 {
        match &self.next {
            &Some(ref next) => next.length() + 1,
            &None => 1,
        }
    }
}
