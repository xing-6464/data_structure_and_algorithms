pub struct Search;

impl Search {
    pub fn linear_search<T: std::cmp::PartialEq>(data: &[T], target: T) -> i32 {
        for (i, v) in data.iter().enumerate() {
            if *v == target {
                return i as i32;
            }
        }
    
        -1
    
    }
}

