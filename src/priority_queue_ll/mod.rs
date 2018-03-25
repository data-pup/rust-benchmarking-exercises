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
pub struct PQueueLl<V, P>
    where P:PartialOrd
{
    head:Option<Box<LlNode<V, P>>>,
    queue_type:QueueType,
}

impl<V, P> PQueueLl<V, P>
    where P: PartialOrd
{
    pub fn new(queue_type:QueueType) -> Self {
        PQueueLl {
            head:None,
            queue_type:queue_type,
        }
    }
}

impl<V, P> Queue<V, P> for PQueueLl<V, P>
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
    use Queue;
    use QueueType;
    use priority_queue_ll;
    use priority_queue_ll::PQueueLl;

    fn create_empty_u32_max_queue() -> PQueueLl<u32, u32> { return PQueueLl::new(QueueType::MaxQueue); }
    fn create_empty_u32_min_queue() -> PQueueLl<u32, u32> { return PQueueLl::new(QueueType::MinQueue); }

    #[test]
    fn create_empty_queues_without_exception() {
        let max_q = create_empty_u32_max_queue();
        let min_q = create_empty_u32_min_queue();
    }

    #[test]
    fn empty_queue_has_length_zero() {
        let (max_q, min_q) = (create_empty_u32_max_queue(), create_empty_u32_min_queue());
        let (max_len, min_len) = (max_q.length(), min_q.length());
        let expected_len = 0;
        assert_eq!(max_len, expected_len);
        assert_eq!(min_len, expected_len);
    }
}
