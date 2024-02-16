pub fn search<T: std::cmp::PartialEq>(data: &[T], target: T) -> isize {
    for (i, v) in data.iter().enumerate() {
        if *v == target {
            return i as isize;
        }
    }

    -1
}
