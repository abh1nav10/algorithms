// IMPLEMENTATION of the unstable version of the selection sort algorithm!

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
