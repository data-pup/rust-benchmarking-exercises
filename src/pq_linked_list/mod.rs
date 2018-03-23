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

impl<'a, V, P> PriorityQueueLL<'a, V, P>
    where V: PartialOrd, P: Unsigned,
{
}

impl<'a, V: 'a, P: 'a> Queue<V, P> for PriorityQueueLL<'a, V, P>
    where V: PartialOrd, P: Unsigned,
{
    fn new(q_type:QueueType) -> Self {
        PriorityQueueLL {
            head: None,
            queue_type: q_type,
        }
    }

    fn push(&mut self, value:V, priority:P) {
        let new_node = PqNode::new(value, priority, None);
        self.head = Some(new_node);
    }

    // fn pop() -> V { }

    fn length(&self) -> u32 { // FIXUP FIXUP FIXUP
        return 1;
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::PartialOrd;
    use {Queue, QueueType};
    use pq_linked_list::PriorityQueueLL;

    #[test]
    fn can_push_onto_empty_queue() {
        let mut q = PriorityQueueLL::new(QueueType::MaxQueue);
        q.push("hello", 0 as u32);
    }

    #[test]
    fn can_use_temp_length_fn() { // FIXUP FIXUP FIXUP
        let q: PriorityQueueLL<u32, u32> = PriorityQueueLL::new(QueueType::MaxQueue);
        let (actual_len, expected_len) = (q.length(), 1);
        assert_eq!(actual_len, expected_len);
    }
}
