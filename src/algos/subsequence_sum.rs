// Kadane's algorithm for Maximum Subsequence Sum!

fn max(first: isize, second: isize) -> isize {
    if first >= second { first } else { second }
}

pub fn max_subsequence_sum(slice: &[isize]) -> isize {
    let mut best_sum = 0;
    let mut sum = 0;
    for element in slice.iter() {
        sum = max(sum + *element, *element);
        best_sum = max(sum, best_sum);
    }
    best_sum
}
