use algorithms::{Heap, Index, MaxHeap, MinHeap};

#[test]
fn max_heap() {
    let mut heap = MaxHeap::new();
    assert!(heap.is_empty());

    heap.push(6);
    heap.push(10);
    heap.push(68);
    heap.push(13);
    heap.push(73);
    heap.push(18);

    let elements = heap.into_iter().collect::<Vec<_>>();

    assert_eq!(elements, [73, 68, 18, 13, 10, 6]);
}

#[test]
fn min_heap() {
    let mut heap = MinHeap::new();
    assert!(heap.is_empty());

    heap.push(6);
    heap.push(10);
    heap.push(68);
    heap.push(13);
    heap.push(73);
    heap.push(18);

    let elements = heap.into_iter().collect::<Vec<_>>();

    assert_eq!(elements, [6, 10, 13, 18, 68, 73]);
}

#[test]
#[should_panic]
fn test_index() {
    let mut heap = MinHeap::new();
    heap.push(5);
    heap.push(10);

    let _: Index = (&heap, 3).try_into().unwrap();
}
