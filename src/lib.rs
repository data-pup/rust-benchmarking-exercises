pub mod pq_binary_heap;
pub mod pq_linked_list;

enum QueueType { MinQueue, MaxQueue }

trait Queue<T> {
    fn new(q_type:QueueType) -> Self;

    // fn push(value:T);
    // fn pop() -> T;
    // fn length();
}

#[cfg(test)]
mod tests {
    use pq_binary_heap;
    use pq_linked_list;

    #[test]
    fn it_works() {
        let bh_hello:String = pq_binary_heap::hello();
        assert_eq!(bh_hello, "Hello from binary tree implementation.");
        let ll_hello:String = pq_linked_list::hello();
        assert_eq!(ll_hello, "Hello from linked list implementation.");
    }
}
