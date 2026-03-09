use algorithms::{
    binary_search, bubble_sort, build_max_heap, counting_sort, heap_sort, insertion_sort,
    max_heapify, max_subsequence_sum, merge_sort, quick_sort, selection_sort,
};

#[test]
fn test_binary_search() {
    let array = [4, 5, 6, 7, 32, 34, 67, 109];
    assert_eq!(4, binary_search(&array[..], 32).unwrap());
}

#[test]
fn test_counting_sort() {
    let mut array = [5, 3, 8, 13, 5, 16, 9, 7];
    let slice = &mut array[..];
    let sorted_vec = counting_sort(slice);
    assert_eq!(sorted_vec, Some([3, 5, 5, 7, 8, 9, 13, 16].to_vec()));
}

#[test]
fn test_insertion_sort() {
    let mut array = [-6, 4, -9, 14, -7, 16];
    let slice = &mut array[..];
    insertion_sort(slice);
    assert_eq!(array, [-9, -7, -6, 4, 14, 16]);
}

#[test]
fn test_selection_sort() {
    let mut array = [4, 6, 2, 8, 9, -4, -7, -2];
    selection_sort(&mut array[..]);
    assert_eq!(array, [-7, -4, -2, 2, 4, 6, 8, 9]);
}

#[test]
fn test_bubble_sort() {
    let mut array = [5, -7, 3, 8, -9, 23, 76];
    let slice = &mut array[..];
    bubble_sort(slice);
    assert_eq!(array, [-9, -7, 3, 5, 8, 23, 76]);
}

#[test]
fn test_heap_sort() {
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

#[test]
fn test_merge_sort() {
    let mut array = [-7, 43, 56, 12, 32, -8, -6, 11, 5];
    let slice = &mut array[..];
    assert_eq!(
        merge_sort(slice),
        Some(vec![-8, -7, -6, 5, 11, 12, 32, 43, 56])
    );
}

#[test]
fn test_quick_sort() {
    let mut array = [4, 5, 6, -3, -7, -1, 17, -13, 12];
    quick_sort(&mut array[..]);
    assert_eq!(array, [-13, -7, -3, -1, 4, 5, 6, 12, 17]);
}

#[test]
fn test_max_subsequence_sum() {
    let array = [3, 5, -5, -8, 8, 5, -9];
    let slice = &array[..];
    let max_subsequence_sum = max_subsequence_sum(slice);
    assert_eq!(13, max_subsequence_sum);
}
