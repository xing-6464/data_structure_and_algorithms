pub struct Sort;

impl Sort {
    pub fn selection_sort<T: std::cmp::PartialOrd>(arr: &mut [T]) {
        // o(n^2)
        for i in 0..arr.len() {
            let mut min_index = i;
            for j in i..arr.len() {
                if arr[j] < arr[min_index] {
                    min_index = j
                }
            }
    
            arr.swap(i, min_index);
        }
    }

    pub fn insertion_sort<T: std::cmp::PartialOrd>(arr: &mut [T]) {
        for i in 0..=arr.len() {
            for j in (1..i).rev().step_by(1) {
                if arr[j] < arr[j-1] {
                    arr.swap(j, j-1);
                } else {
                    break;
                }
            }
        }
    }
}
