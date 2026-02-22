// Kadane's algorithm

// find the maximum subsequence sum

#![allow(dead_code)]

fn max(first: isize, second: isize) -> isize {
    if first >= second { first } else { second }
}

fn subsequence_sum(slice: &[isize]) -> isize {
    let mut best_sum = 0;
    let mut sum = 0;
    for i in 0..slice.len() {
        sum = max(sum + slice[i], slice[i]);
        best_sum = max(sum, best_sum);
    }
    best_sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let array = [3, 5, -5, -8, 8, 5, -9];
        let slice = &array[..];
        let max_subsequence_sum = subsequence_sum(slice);
        assert_eq!(13 as isize, max_subsequence_sum);
    }
}
