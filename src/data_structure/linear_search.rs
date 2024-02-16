pub fn search(data: &[isize], target: isize) -> isize {
    for (i, &v) in data.iter().enumerate() {
        if v == target {
            return i as isize;
        }
    }

    -1
}
