// Implementation of the bubble sort algorithm!

pub fn bubble_sort(slice: &mut [isize]) {
    for i in 0..slice.len() {
        for j in (i + 1)..slice.len() {
            if slice[i] > slice[j] {
                slice.swap(i, j);
            }
        }
    }
}
