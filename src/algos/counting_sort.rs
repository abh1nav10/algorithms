// Implementation of the counting sort algorithm!

// Good for a sequence containing small range of positive integer input values..

fn largest_element(slice: &mut [usize]) -> usize {
    let mut largest = 0;
    for element in slice.iter() {
        if *element > largest {
            largest = *element;
        }
    }
    largest
}

fn sort(slice: &mut [usize]) -> Vec<usize> {
    let largest_element = largest_element(slice);
    let mut new_vec: Vec<usize> = vec![0; largest_element + 1];
    for element in slice.iter() {
        new_vec[*element] += 1;
    }
    let mut sorted_vec = Vec::with_capacity(slice.len());
    for (index, element) in new_vec.iter().enumerate() {
        let occurence = *element;
        for _ in 0..occurence {
            sorted_vec.push(index);
        }
    }
    sorted_vec
}

pub fn counting_sort(slice: &mut [usize]) -> Option<Vec<usize>> {
    if slice.is_empty() {
        None
    } else {
        let sorted = sort(slice);
        Some(sorted)
    }
}
