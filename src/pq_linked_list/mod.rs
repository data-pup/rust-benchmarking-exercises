/// Temporary function.
pub fn hello() -> String {
    return String::from("Hello from linked list implementation.");
}

use {Queue, QueueType};

struct LLNode<'a, T: 'a> {
    pub value:T,
    pub next:&'a LLNode<'a, T>,
}

pub struct PriorityQueueLL<'a, T: 'a, F>
    where F: Fn(T, T) -> bool
{
    pub head:LLNode<'a, T>,
    compare: F,
}

impl<'a, T: 'a, F> Queue<T> for PriorityQueueLL<'a, T, F>
    where F: Fn(T, T) -> bool
{
    fn new(q_type:QueueType) -> Self {
        PriorityQueueLL {
        }
    }

    // fn push(value:T) { }
    // fn pop() -> T { }
    // fn length();
}
