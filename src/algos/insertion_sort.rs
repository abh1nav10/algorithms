// Implementation of the Insertion Sort sorting algorithm!

fn sort(slice: &mut [isize]) {
    for i in 0..slice.len() {
        let mut index = i;
        for j in (0..i).rev() {
            if slice[j] > slice[index] {
                slice.swap(index, j);
                index = j;
            }
        }
    }
}

pub fn insertion_sort(slice: &mut [isize]) {
    if slice.is_empty() || slice.len() == 1 {
        return;
    }
    sort(slice);
}
