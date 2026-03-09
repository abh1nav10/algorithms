// Implementation of the binary search algorithm!

// The caller is required to sort the array first as binary search works on a sorted array...
// making the function sort the array as a part of its implementation is not good as we would be
// returning an index that might lead to a different value in the unsorted array...therefore it is
// better for the user to sort it first to avoid confusion..

use std::cmp::Ordering;

fn search<T: Ord>(slice: &[T], element: T) -> Option<usize> {
    let mid_index = slice.len() / 2;
    match element.cmp(&slice[mid_index]) {
        Ordering::Equal => Some(mid_index),
        Ordering::Greater => {
            let result = binary_search(&slice[(mid_index + 1)..], element);
            result.map(|index| mid_index + index + 1)
        }
        Ordering::Less => binary_search(&slice[..mid_index], element),
    }
}

pub fn binary_search<T: Ord>(slice: &[T], element: T) -> Option<usize> {
    if slice.is_empty() {
        return None;
    }
    search(slice, element)
}
