use std::cmp::PartialOrd;

use Queue;
use QueueType;

/// Single node for the priority queue linked list implementation.
struct LlNode<V, P>
    where P:PartialOrd
{
    value:V,
    priority:P,
    next:Option<Box<LlNode<V, P>>>,
}

/// Priority Queue, using a linked list implementation.
pub struct PriorityQueueLinkedList<V, P>
    where P:PartialOrd
{
    head:Option<Box<LlNode<V, P>>>,
    queue_type:QueueType,
}

impl<V, P> PriorityQueueLinkedList<V, P>
    where P: PartialOrd
{
    pub fn new(queue_type:QueueType) -> Self {
        PriorityQueueLinkedList {
            head:None,
            queue_type:queue_type,
        }
    }
}

impl<V, P> Queue<V, P> for PriorityQueueLinkedList<V, P>
    where P: PartialOrd
{
    fn push(&mut self, value:V, priority:P) {
    }

    fn pop(&mut self) -> Option<V> {
        None // FIXUP
    }

    fn length(&self) -> u32 {
        0 // FIXUP
    }
}

#[cfg(test)]
mod tests {
    use QueueType;
    use priority_queue_ll;
    use priority_queue_ll::PriorityQueueLinkedList;

    #[test]
    fn create_empty_queues_without_exception() {
        let min_q:PriorityQueueLinkedList<u32, u32> =
            PriorityQueueLinkedList::new(QueueType::MinQueue);
        let max_q:PriorityQueueLinkedList<u32, u32> =
            PriorityQueueLinkedList::new(QueueType::MaxQueue);
    }
}
