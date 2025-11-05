// implementation of the unstable version of the selection sort algorithm

#![allow(dead_code)]

fn sort<T: Ord>(slice: &mut [T]) {
    for i in 0..slice.len() {
        let mut shortest_index = i;
        for j in (i + 1)..slice.len() {
            if slice[j] < slice[shortest_index] {
                shortest_index = j;
            }
        }
        slice.swap(i, shortest_index);
    }
}

pub fn selection_sort<T: Ord>(slice: &mut [T]) {
    if slice.is_empty() || slice.len() == 1 {
        return;
    }
    sort(slice);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut array = [4, 6, 2, 8, 9, -4, -7, -2];
        selection_sort(&mut array[..]);
        assert_eq!(array, [-7, -4, -2, 2, 4, 6, 8, 9]);
    }
}
