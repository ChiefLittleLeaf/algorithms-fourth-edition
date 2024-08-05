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
    fn t1_merge_sort() {
        let sorted = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut arr = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        merge_sort(&mut arr);
        assert_eq!(sorted, arr);
    }
    #[test]
    fn t2_merge_sort() {
        let sorted = [0, 11, 328, 499, 500, 501, 547, 654, 777, 1000];
        let mut arr = [1000, 328, 777, 547, 654, 500, 501, 499, 0, 11];
        merge_sort(&mut arr);
        assert_eq!(sorted, arr);
    }
    #[test]
    fn t3_merge_sort() {
        let mut arr = vec![0_i32; 1_000_000];
        rand::thread_rng().fill(&mut arr[..]);
        let mut sorted = arr.clone();
        sorted.sort();
        merge_sort(&mut arr);
        assert_eq!(sorted, arr);
    }

    #[test]
    fn t4_merge_sort() {
        let mut arr = vec![0_i32; 10_000_000];
        rand::thread_rng().fill(&mut arr[..]);
        let mut sorted = arr.clone();
        sorted.sort();
        merge_sort(&mut arr);
        assert_eq!(sorted, arr);
    }
    #[test]
    fn t5_merge_sort() {
        let mut arr = vec![0_i32; 20_000_000];
        rand::thread_rng().fill(&mut arr[..]);
        let mut sorted = arr.clone();
        sorted.sort();
        merge_sort(&mut arr);
        assert_eq!(sorted, arr);
    }
}
