use super::heap::MinHeap;

#[derive(Debug)]
pub struct BinaryHeap<T>
where T: PartialOrd
{
    arr: Vec<T>,
    capacity: usize,
}

impl<T> BinaryHeap<T>
where T: PartialOrd
{
    pub fn new(capacity: usize) -> Self {
        BinaryHeap {
            arr: Vec::with_capacity(capacity),
            capacity,
        }
    }

    fn upheap(&mut self, i: usize) {
        if i == 0 {
            return;
        }

        let parent_index = (i - 1) / 2;
        let parent = &self.arr[parent_index];
        let child = &self.arr[i];
        if child < parent {
            self.arr.swap(parent_index, i);
        }

        self.upheap(parent_index);
    }

    fn downheap(&mut self, i: usize) {
        let c1_index = i * 2 + 1;
        if c1_index >= self.arr.len() {
            // no children
            return;
        }

        let parent = &self.arr[i];

        let c2_index = i * 2 + 2;

        let mut min_child_index = c1_index;
        if c2_index < self.arr.len() {
            // has two children - choose smaller one
            if &self.arr[c2_index] < &self.arr[c1_index] {
                min_child_index = c2_index;
            }
        }

        if &self.arr[min_child_index] < parent {
            self.arr.swap(i, min_child_index);
            self.downheap(min_child_index);
        }
    }
}

impl<T> MinHeap<T> for BinaryHeap<T>
where T: PartialOrd
{
    fn insert(&mut self, t: T) {
        if self.arr.len() == self.capacity {
            panic!("capacity exceeded");
        }

        self.arr.push(t);
        self.upheap(self.arr.len() - 1);
    }

    fn peek(&self) -> Option<&T> {
        self.arr.first()
    }

    fn pop(&mut self) -> Option<T> {
        if self.arr.is_empty() {
            return None;
        }

        let t = self.arr.swap_remove(0);
        self.downheap(0);
        Some(t)
    }

    fn size(&self) -> usize {
        self.arr.len()
    }
}

impl<T> From<Vec<T>> for BinaryHeap<T>
where T: PartialOrd
{
    fn from(value: Vec<T>) -> Self {
        let mut heap = Self::new(value.len());
        for val in value {
            heap.insert(val);
        }

        heap
    }
}
