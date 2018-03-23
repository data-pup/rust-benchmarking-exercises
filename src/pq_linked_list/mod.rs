/// Temporary function.
pub fn hello() -> String {
    return String::from("Hello from linked list implementation.");
}

use std::cmp::PartialOrd;
use num::Unsigned;
use {Queue, QueueType};
mod node;
use pq_linked_list::node::PqNode;

pub struct PriorityQueueLL<'a, V: 'a, P: 'a>
    where V: PartialOrd, P: Unsigned,
{
    pub head:Option<PqNode<'a, V, P>>,
    queue_type: QueueType,
}

// impl<'a, V, P> PriorityQueueLL<'a, V, P>
//     where V: PartialOrd, P: Unsigned,
// {
// }

impl<'a, V: 'a, P: 'a> Queue<V> for PriorityQueueLL<'a, V, P>
    where V: PartialOrd, P: Unsigned,
{
    fn new(q_type:QueueType) -> Self {
        PriorityQueueLL {
            head: None,
            queue_type: q_type,
        }
    }
    // fn push(value:V) { }
    // fn pop() -> V { }
    // fn length();
}