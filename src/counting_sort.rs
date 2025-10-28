// implementation of the counting sort algorithm

// good for a sequence containing small range of positive integer input values

#![allow(dead_code)]

fn largest_element(slice: &mut [usize]) -> usize {
    let mut largest = 0;
    for i in 0..slice.len() {
        if slice[i] > largest {
            largest = slice[i];
        }
    }
    largest
}

fn sort(slice: &mut [usize]) -> Vec<usize> {
    let largest_element = largest_element(slice);
    let mut new_vec: Vec<usize> = vec![0; largest_element + 1];
    for i in 0..slice.len() {
        new_vec[slice[i]] += 1;
    }
    let mut sorted_vec = Vec::with_capacity(slice.len());
    for element in 0..new_vec.len() {
        let occurence = new_vec[element];
        for _ in 0..occurence {
            sorted_vec.push(element);
        }
    }
    sorted_vec
}

fn counting_sort(slice: &mut [usize]) -> Option<Vec<usize>> {
    if slice.is_empty() {
        None
    } else {
        let sorted = sort(slice);
        Some(sorted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tests() {
        let mut array = [5, 3, 8, 13, 5, 16, 9, 7];
        let slice = &mut array[..];
        let sorted_vec = counting_sort(slice);
        assert_eq!(sorted_vec, Some([3, 5, 5, 7, 8, 9, 13, 16].to_vec()));
    }
}
