pub fn max_heapify<T: Ord>(slice: &mut [T], index: usize, heap_size: usize) {
    let mut index = index;
    loop {
        let left_child = index * 2 + 1;
        let right_child = index * 2 + 2;
        let mut greatest = index;
        if left_child < heap_size && slice[index] < slice[left_child] {
            greatest = left_child;
        }
        if right_child < heap_size && slice[greatest] < slice[right_child] {
            greatest = right_child;
        }
        if greatest != index {
            slice.swap(index, greatest);
            index = greatest;
        } else {
            break;
        }
    }
}

// Don't like recursion!
pub fn build_max_heap<T: Ord>(slice: &mut [T]) -> &mut [T] {
    let heap_size = slice.len();
    if heap_size <= 1 {
        return slice;
    }
    let non_leaf = heap_size / 2 - 1;
    for i in (0..=non_leaf).rev() {
        let mut index = i;
        loop {
            let left_child = index * 2 + 1;
            let right_child = index * 2 + 2;
            let mut greatest = index;
            if left_child < heap_size && slice[index] < slice[left_child] {
                greatest = left_child;
            }
            if right_child < heap_size && slice[greatest] < slice[right_child] {
                greatest = right_child;
            }
            if greatest != index {
                slice.swap(index, greatest);
                index = greatest;
            } else {
                break;
            }
        }
    }
    slice
}

pub fn heap_sort<T: Ord>(slice: &mut [T]) {
    let slice = build_max_heap(slice);
    let mut heap_size = slice.len() - 1;
    slice.swap(0, heap_size);
    for _ in (0..(slice.len() - 2)).rev() {
        max_heapify(&mut slice[..], 0, heap_size);
        heap_size -= 1;
        slice.swap(0, heap_size);
    }
}
