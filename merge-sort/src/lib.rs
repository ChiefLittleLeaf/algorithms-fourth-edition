pub fn merge_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let mid = arr.len() / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let mut temp = arr.to_vec();
    merge(&arr[..mid], &arr[mid..], &mut temp[..]);

    arr.copy_from_slice(&temp);
}

pub fn merge(left: &[i32], right: &[i32], output: &mut [i32]) {
    let mut left_idx = 0;
    let mut right_idx = 0;
    let mut output_idx = 0;

    while left_idx < left.len() && right_idx < right.len() {
        if left[left_idx] <= right[right_idx] {
            output[output_idx] = left[left_idx];
            left_idx += 1;
        } else {
            output[output_idx] = right[right_idx];
            right_idx += 1;
        }
        output_idx += 1;
    }

    if left_idx < left.len() {
        output[output_idx..].copy_from_slice(&left[left_idx..]);
    } else {
        output[output_idx..].copy_from_slice(&right[right_idx..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn merge_sort_1() {
        let mut arr: Vec<i32> = (0..1)
            .map(|_| rand::thread_rng().gen_range(0..1_000_000))
            .collect();
        let mut sorted = arr.clone();
        sorted.sort();
        merge_sort(&mut arr);
        assert_eq!(sorted, arr);
    }
    #[test]
    fn merge_sort_10() {
        let mut arr: Vec<i32> = (0..10)
            .map(|_| rand::thread_rng().gen_range(0..1_000_000))
            .collect();
        let mut sorted = arr.clone();
        sorted.sort();
        merge_sort(&mut arr);
        assert_eq!(sorted, arr);
    }
    #[test]
    fn merge_sort_1_000_000() {
        let mut arr: Vec<i32> = (0..1_000_000)
            .map(|_| rand::thread_rng().gen_range(0..1_000_000))
            .collect();
        let mut sorted = arr.clone();
        sorted.sort();
        merge_sort(&mut arr);
        assert_eq!(sorted, arr);
    }

    #[test]
    fn merge_sort_10_000_000() {
        let mut arr: Vec<i32> = (0..10_000_000)
            .map(|_| rand::thread_rng().gen_range(0..1_000_000))
            .collect();
        let mut sorted = arr.clone();
        sorted.sort();
        merge_sort(&mut arr);
        assert_eq!(sorted, arr);
    }
    #[test]
    fn merge_sort_20_000_000() {
        let mut arr: Vec<i32> = (0..20_000_000)
            .map(|_| rand::thread_rng().gen_range(0..1_000_000))
            .collect();
        let mut sorted = arr.clone();
        sorted.sort();
        merge_sort(&mut arr);
        assert_eq!(sorted, arr);
    }
}
