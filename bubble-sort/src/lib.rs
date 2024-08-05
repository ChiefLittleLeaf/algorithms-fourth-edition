pub fn bubble_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                println!("swapping {} with {}", arr[j], arr[j + 1]);
                let tmp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = tmp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn bubble_sort_1() {
        let mut arr = vec![0_i32; 1];
        rand::thread_rng().fill(&mut arr[..]);
        let mut sorted = arr.clone();
        sorted.sort();
        bubble_sort(&mut arr);
        assert_eq!(sorted, arr);
    }

    #[test]
    fn bubble_sort_10() {
        let mut arr = vec![0_i32; 10];
        rand::thread_rng().fill(&mut arr[..]);
        let mut sorted = arr.clone();
        sorted.sort();
        bubble_sort(&mut arr);
        assert_eq!(sorted, arr);
    }

    #[test]
    fn bubble_sort_100() {
        let mut arr = vec![0_i32; 100];
        rand::thread_rng().fill(&mut arr[..]);
        let mut sorted = arr.clone();
        sorted.sort();
        bubble_sort(&mut arr);
        assert_eq!(sorted, arr);
    }

    #[test]
    fn bubble_sort_1_000() {
        let mut arr = vec![0_i32; 1_000];
        rand::thread_rng().fill(&mut arr[..]);
        let mut sorted = arr.clone();
        sorted.sort();
        bubble_sort(&mut arr);
        assert_eq!(sorted, arr);
    }
    #[test]
    fn bubble_sort_10_000() {
        let mut arr: Vec<i32> = (0..10_000)
            .map(|_| rand::thread_rng().gen_range(0..10_000))
            .collect();
        let mut sorted = arr.clone();
        sorted.sort();
        bubble_sort(&mut arr);
        assert_eq!(sorted, arr);
    }
}
