mod heap;

use crate::heap::{binary_heap::BinaryHeap, heap::MinHeap};

fn main() {
    const N: u32 = 50;

    let mut heap = BinaryHeap::new(N as usize);
    let a = [0u8; N as usize];

    loop {
        for _ in 0..N {
            let rand = rand::random::<u32>() % N;
            heap.insert(rand);
        }

        for _ in 0..N {
            let val = heap.pop().unwrap();
            eprintln!("val = {:?}", val);
        }
    }
}
