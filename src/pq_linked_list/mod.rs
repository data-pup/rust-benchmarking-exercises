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
    // fn get_tail(&self) -> Option<&PqNode<V, P>> {
    //     match &self.head {
    //         &Some(ref head) => {
    //             let mut curr = &head;
    //             // while (curr.next).is_some() { curr = &curr.next.unwrap(); }
    //             Some(&curr)
    //         },
    //         &None => None,
    //     }
    // }

    // fn get_insert_pos(&self, priority:P) -> (Option<& PqNode<V, P>>,
    //                                          Option<& PqNode<V, P>>) {
    //     if self.head.is_some() {
    //         let mut prev = self.head.unwrap();
    //         while prev.next.is_some() {
    //             prev = prev.next.unwrap();
    //         }
    //         (Some(&prev), None)
    //     } else {
    //         (None, None)
    //     }
    // }
}

impl<'a, V, P> Queue<V, P> for PriorityQueueLL<'a, V, P>
    where V: PartialOrd, P: Unsigned,
{
    fn new(q_type:QueueType) -> Self {
        PriorityQueueLL {
            head: None,
            queue_type: q_type,
        }
    }

    fn push(&mut self, value:V, priority:P) {
        match &mut self.head {
            &mut Some(_) => {
                return;
            },
            &mut None => self.head = Some(PqNode::new(value, priority, None)),
        };
    }

    // fn pop() -> V { }

    fn length(&self) -> u32 {
        match &self.head {
            &Some(ref head) => {
                let (mut i, mut curr) = (1 as u32, head);
                while (curr.next).is_some() {
                    i += 1;
                    curr = &curr.next.unwrap();
                }
                return i;
            },
            &None => 0 as u32,
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
