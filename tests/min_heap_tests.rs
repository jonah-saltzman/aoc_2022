use aoc_2022::min_heap::MinHeap;

#[test]
fn sorts_numbers() {
    let mut heap = MinHeap::<i32>::new();
    let nums = vec![3, 1, 2, 4, 6, 5, 0];
    for n in nums {
        heap.push(n);
    }
    for expected in 0..=6 {
        assert_eq!(expected, heap.pop().unwrap());
    }
}
