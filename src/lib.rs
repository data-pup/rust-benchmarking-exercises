enum QueueType { MinQueue, MaxQueue }

trait Queue<V, P>
    where P: PartialOrd
{
    fn new(q_type:QueueType) -> Self;
    fn push(&mut self, value:V, priority:P);
    fn pop(&mut self) -> Option<V>;
    fn length(&self) -> u32;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
