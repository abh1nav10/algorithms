// implementation of the insertion sort sorting algorithm

#![allow(dead_code)]

fn sort(slice: &mut [isize]) {
    for i in 0..slice.len() {
        let mut index = i;
        for j in (0..i).into_iter().rev() {
            if slice[j] > slice[index] {
                let current = slice[j];
                slice[j] = slice[index];
                slice[index] = current;
                index = j;
            }
        }
    }
}

fn insertion_sort(slice: &mut [isize]) {
    if slice.is_empty() || slice.len() == 1 {
        return;
    }
    sort(slice);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tests() {
        let mut array = [-6, 4, -9, 14, -7, 16];
        let slice = &mut array[..];
        insertion_sort(slice);
        assert_eq!(array, [-9, -7, -6, 4, 14, 16]);
    }
}
