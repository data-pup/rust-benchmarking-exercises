/// Temporary function.
pub fn hello() -> String {
    return String::from("Hello from linked list implementation.");
}

use std::cmp::PartialOrd;
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

impl<V, P> PriorityQueueLL<V, P>
    where P: PartialOrd, P: Unsigned
{
    /// This private function is used to find the insert position for a new node.
    fn get_insert_pos(&self, priority:&P) -> &Option<Box<PqNode<V, P>>> {
        // This closure helps determine if the insert position has been found.
        let should_continue = |curr_priority:&P, priority:&P| -> bool {
            match self.queue_type {
                QueueType::MaxQueue => curr_priority > &priority,
                QueueType::MinQueue => curr_priority < &priority,
            }
        };

        // Return None if no head exists, otherwise, borrow the head.
        if self.head.is_none() { return &None; }
        let mut curr: &Option<Box<PqNode<V, P>>> = &self.head;

        // Return None if the new node should be inserted as the new head.
        let curr_priority:&P = &curr.as_ref().unwrap().priority;
        if !should_continue(&curr_priority, &priority) { return &None; }

        loop { // Loop until the insert position (or tail node) is found.
            let curr_node = curr.as_ref().unwrap(); // Unwrap the node.
            if curr_node.next.is_none() { break; }  // Break if at tail node.
            // Unwrap the priority at the next node, check whether to continue.
            let next_priority:&P = &curr_node.next.as_ref().unwrap().priority;
            if !should_continue(&next_priority, &priority) { break; }
            curr = &curr_node.next; // Otherwise, continue to the next node.
        }   // After the loop breaks, return the given optional pointer.
        return curr;
    }
}

impl<V, P> Queue<V, P> for PriorityQueueLL<V, P>
    where P: PartialOrd, P: Unsigned
{
    fn new(q_type:QueueType) -> Self {
        PriorityQueueLL {
            head: None,
            queue_type: q_type,
        }
    }

    /// FIXUP needed.
    fn push(&mut self, value:V, priority:P) {
        let parent:&Option<Box<PqNode<V, P>>> = self.get_insert_pos(&priority);
        match parent {
            &Some(ref prev) => {  }
        //         prev.next = Some(Box::new(PqNode::new(value, priority, None)));
            &None => {  }
            // self.head = Some(Box::new(PqNode::new(value, priority, None))),
        };
    }

    // fn pop() -> V { }

    fn length(&self) -> u32 {
        match self.head {
            Some(ref head) => head.length(),
            None => 0 as u32,
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
