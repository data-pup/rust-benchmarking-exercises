use std::cmp::PartialOrd;
use num::Unsigned;

/// This struct represents a node in a linked list implementation of a priority
/// queue. The V type represents the type of value stored in the node, and the
/// P type represents the kind of unsigned value that is being used to denote
/// the priority of the node.
pub struct PqNode<'a, V: 'a, P: 'a>
    where V: PartialOrd, P: Unsigned,
{
    pub value:V,
    pub priority:P,
    pub next:Option<&'a PqNode<'a, V, P>>,
}

impl<'a, V: 'a, P: 'a> PqNode<'a, V, P>
    where V: PartialOrd, P: Unsigned,
{
    pub fn new(value:V, priority:P, next:Option<&'a PqNode<'a, V, P>>) -> Self {
        PqNode { value, priority, next }
    }

    pub fn get_tail(&self) -> Option<&PqNode<'a, V, P>> {
        match self.next {
            Some(next) => next.get_tail(),
            None => Some(&self),
        }
    }
}
