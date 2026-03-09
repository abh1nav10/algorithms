// Implementation of the merge sort algorithm!

fn sort(slice: &mut [isize]) -> Vec<isize> {
    let mid_index = slice.len() / 2;
    let (first, second) = slice.split_at_mut(mid_index);
    let mut first = if first.len() != 1 {
        sort(first)
    } else {
        first.to_vec()
    };
    let mut second = if second.len() != 1 {
        sort(second)
    } else {
        second.to_vec()
    };
    merge(&mut first, &mut second)
}

fn merge<'a>(first: &'a mut [isize], second: &'a mut [isize]) -> Vec<isize> {
    let len = first.len() + second.len();
    let mut vec = Vec::with_capacity(len);
    let mut pointer1: usize = 0;
    let mut pointer2: usize = 0;
    loop {
        if first[pointer1] <= second[pointer2] {
            vec.push(first[pointer1]);
            pointer1 += 1;
        } else {
            vec.push(second[pointer2]);
            pointer2 += 1;
        }
        if first.len() - pointer1 == 0 {
            for element in second[pointer2..].iter() {
                vec.push(*element);
            }
            break;
        }
        if second.len() - pointer2 == 0 {
            for element in first[pointer1..].iter() {
                vec.push(*element);
            }
            break;
        }
    }
    vec
}

pub fn merge_sort(slice: &mut [isize]) -> Option<Vec<isize>> {
    if slice.is_empty() || slice.len() == 1 {
        return None;
    }
    let sorted = sort(slice);
    Some(sorted)
}
