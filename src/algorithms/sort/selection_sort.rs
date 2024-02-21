pub fn selection_sort(arr: &mut [isize]) {
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