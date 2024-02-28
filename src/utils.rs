use std::time::{SystemTime, UNIX_EPOCH};

use rand::Rng;

use crate::algorithms::sort::Sort;

pub struct ArrayGenerator;
pub struct SortingHelper;

impl ArrayGenerator {
    pub fn generator_ordered_array(n: isize) -> Vec<isize> {
        let mut res_array = Vec::new();

        for i in 0..n {
            res_array.push(i);
        }

        res_array
    }


    pub fn generate_random_array(n: i32, bound: i32) -> Vec<i32> {
        let mut arr: Vec<i32> = Vec::new();
        for i in 0..n {
            arr.push(rand::thread_rng().gen_range(0..bound));
        }
        arr
    }
}

impl SortingHelper {
    pub fn isSorted<T: PartialOrd>(arr: &[T]) -> bool {
        for i in 1..arr.len() {
            if arr[i-1] > arr[i] {
                return false;
            }
        }
        true
    }

    pub fn sortTest<T: PartialOrd>(sort_name: &str, arr: &mut [T]) {

        let start_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("failed to get timestamp").as_secs_f64();
        match sort_name {
            "selection_sort" => Sort::selection_sort(arr),
            _ => unreachable!()
        }
        let end_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("failed to get timestamp").as_secs_f64();

        let time = end_time - start_time;

        if !SortingHelper::isSorted(arr) {
            panic!("{} failed", sort_name);
        }

        println!("{}, n = {}:  {}s", sort_name, arr.len(), time);
    }
}
