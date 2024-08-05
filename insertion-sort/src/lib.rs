pub fn insertion_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        let key = arr[i];
        let mut j = i as isize - 1;

        while j >= 0 && arr[j as usize] > key {
            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1;
        }
        arr[(j + 1) as usize] = key;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut arrays = vec![
            [57, 2, 48, 32, 5],
            [10, 100, 59, 77, 26],
            [5, 4, 3, 2, 1],
            [100, 10, 40, 77, 52],
            [50, 30, 40, 10, 20],
            [96, 100, 99, 98, 97],
        ];
        let sorted_tests = vec![
            [2, 5, 32, 48, 57],
            [10, 26, 59, 77, 100],
            [1, 2, 3, 4, 5],
            [10, 40, 52, 77, 100],
            [10, 20, 30, 40, 50],
            [96, 97, 98, 99, 100],
        ];
        if arrays.len() == sorted_tests.len() {
            for i in 0..sorted_tests.len() {
                insertion_sort(&mut arrays[i]);
            }
            assert_eq!(sorted_tests, arrays);
        } else {
            assert!(
                false,
                "arrays length: {} and sorted arrays length: {} do not match",
                arrays.len(),
                sorted_tests.len()
            )
        }
    }
}
