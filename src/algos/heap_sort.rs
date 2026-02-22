#![allow(unused)]

fn max_heapify<T: Ord>(slice: &mut [T], index: usize, heap_size: usize) {
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
fn build_max_heap<T: Ord>(slice: &mut [T]) -> &mut [T] {
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

fn heap_sort<T: Ord>(slice: &mut [T]) {
    let slice = build_max_heap(slice);
    let mut heap_size = slice.len() - 1;
    slice.swap(0, heap_size);
    for _ in (0..(slice.len() - 2)).rev() {
        max_heapify(&mut slice[..], 0, heap_size);
        heap_size -= 1;
        slice.swap(0, heap_size);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut array = [65, 32, 56, 27, 44];
        let len = array.len();

        max_heapify(&mut array[..], 1, len);
        assert_eq!(array, [65, 44, 56, 27, 32]);

        let mut array = [44, 66, 33, 24, 89, 67, 12, 34];

        let _ = build_max_heap(&mut array);
        assert_eq!(array, [89, 66, 67, 34, 44, 33, 12, 24]);

        let mut array = [44, 66, 33, 24, 89, 67, 12, 34];

        heap_sort(&mut array[..]);
        assert_eq!(array, [12, 24, 33, 34, 44, 66, 67, 89]);
    }
}
