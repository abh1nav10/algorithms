// implementation of the binary search algorithm

// the caller is required to sort the array first as binary search works on a sorted array...
// making the function sort the array as a part of its implementation is not good as we would be
// returning an index that might lead to a different value in the unsorted array...therefore it is
// better for the user to sort it first to avoid confusion

#![allow(dead_code)]
use std::cmp::Ordering;

fn search(slice: &[isize], element: isize) -> Option<usize> {
    let mid_index = slice.len() / 2;
    match element.cmp(&slice[mid_index]) {
        Ordering::Equal => {
            return Some(mid_index);
        }
        Ordering::Greater => {
            let result = binary_search(&slice[(mid_index + 1)..], element);
            if let Some(index) = result {
                return Some(mid_index + index + 1);
            } else {
                return None;
            }
        }
        Ordering::Less => binary_search(&slice[..mid_index], element),
    }
}

fn binary_search(slice: &[isize], element: isize) -> Option<usize> {
    if slice.is_empty() {
        return None;
    }
    search(slice, element)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let array = [4, 5, 6, 7, 32, 34, 67, 109];
        assert_eq!(4 as usize, binary_search(&array[..], 32).unwrap());
    }
}
