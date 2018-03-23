use num::Unsigned;

/// This struct represents a node in a linked list implementation of a priority
/// queue. The V type represents the type of value stored in the node, and the
/// P type represents the kind of unsigned value that is being used to denote
/// the priority of the node.
pub struct PqNode<'a, V: 'a, P: 'a>
    where P : Unsigned
{
    pub value:V,
    pub priority:P,
    pub next:&'a PqNode<'a, V, P>,
}
