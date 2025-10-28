// implementation of the bubble sort algorithm

#[allow(dead_code)]
fn bubble_sort(slice: &mut [isize]) {
    for i in 0..slice.len() {
        for j in (i + 1)..slice.len() {
            if slice[i] > slice[j] {
                let current = slice[i];
                slice[i] = slice[j];
                slice[j] = current;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tests() {
        let mut array = [5, -7, 3, 8, -9, 23, 76];
        let slice = &mut array[..];
        bubble_sort(slice);
        assert_eq!(array, [-9, -7, 3, 5, 8, 23, 76]);
    }
}
