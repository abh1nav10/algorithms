// implementation of the quick sort algorithm using the lomuto partition algorithm

#![allow(dead_code)]

fn sort(slice: &mut [isize]) {
    let mut pivot_index = slice.len() - 1;
    let mut boundary_index: isize = -1;
    let mut current_index = 0;
    while current_index != pivot_index {
        if slice[current_index] < slice[pivot_index] {
            boundary_index += 1;
            slice.swap(boundary_index as usize, current_index);
        }
        current_index += 1;
    }
    slice.swap(pivot_index, (boundary_index + 1) as usize);
    pivot_index = (boundary_index + 1) as usize;
    let (first, second) = slice.split_at_mut(pivot_index);
    if first.len() > 1 {
        sort(first);
    }
    if second.len() > 2 {
        sort(&mut second[1..]);
    }
}

fn quick_sort(slice: &mut [isize]) {
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
        let mut array = [4, 5, 6, -3, -7, -1, 17, -13, 12];
        quick_sort(&mut array[..]);
        assert_eq!(array, [-13, -7, -3, -1, 4, 5, 6, 12, 17]);
    }
}
