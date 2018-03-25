/// Temporary function.
pub fn hello() -> String {
    return String::from("Hello from linked list implementation.");
}

use std::cmp::PartialOrd;
use std::option::Option;
use std::boxed::Box;

use num::Unsigned;
use {Queue, QueueType};
mod node;
use pq_linked_list::node::PqNode;

pub struct PriorityQueueLL<V, P>
    where P: PartialOrd, P: Unsigned
{
    pub head:Option<Box<PqNode<V, P>>>,
    queue_type: QueueType,
}

impl<V, P> Queue<V, P> for PriorityQueueLL<V, P>
    where P: PartialOrd, P: Unsigned
{
    /// Creates a new priority queue.
    fn new(q_type:QueueType) -> Self {
        PriorityQueueLL {
            head: None,
            queue_type: q_type,
        }
    }

    /// Pushes a new value-priority pair into the correct place in the queue.
    fn push(&mut self, value:V, priority:P) {
        match self.is_new_head(&priority) {
            false => {
                let mut prev = self.head.as_ref().unwrap();
                let next = &prev.next;
                loop {
                    break;
                }
                let new_node = Some(Box::new(PqNode::new(value, priority, prev)));
            }
            true => {
                self.head = Some(Box::new(PqNode::new(value, priority, None)));
            },
        }
    }

    // PUSH Notes:
    // 1. Empty queue (self.head -> None)   =>   New node is self.head
    // 2. Check that self.head.priority more important than new priority.
    //      If not, new node is new head, set next member and self.head accordingly.
    // 3. Traverse until the correct next node, or tail is found.

    fn length(&self) -> u32 {
        match self.head {
            Some(ref head) => head.length(),
            None => 0 as u32,
        }
    }
}

impl<V, P> PriorityQueueLL<V, P>
    where P: PartialOrd, P: Unsigned
{
    /// Returns true if a new node with the given priority should be the new
    /// head for the given type of priority queue.
    fn is_new_head(&self, priority:&P) -> bool {
        if self.head.is_none() { return true; }
        else {
            let head_node = self.head.as_ref().unwrap();
            let head_priority = &head_node.priority;
            let should_be_head = !self.should_continue(priority, head_priority);
            return should_be_head;
        }
    }

    /// This helper function is used to determine whether to progress further
    /// when identifying an insert position, and whether or not a new node
    /// should be placed at the head of the queue.
    fn should_continue(&self, priority:&P, next_priority:&P) -> bool {
        return match self.queue_type {
            QueueType::MaxQueue => priority > next_priority,
            QueueType::MinQueue => priority < next_priority,
        }
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
    fn length_test() {
        let mut expected_len:u32 = 0;
        let mut q = PriorityQueueLL::new(QueueType::MaxQueue);
        assert_eq!(q.length(), expected_len);
        q.push(1 as u32, 1 as u32);
        expected_len += 1;
        assert_eq!(q.length(), expected_len);
    }
}
