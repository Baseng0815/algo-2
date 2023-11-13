pub trait MinHeap<T> {
    fn insert(&mut self, t: T);
    fn peek(&self) -> Option<&T>;
    fn pop(&mut self) -> Option<T>;
    fn size(&self) -> usize;
}

pub trait AdressableHeap<T> {
    fn remove(&mut self,)
}
