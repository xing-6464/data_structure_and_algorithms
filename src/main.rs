use std::time::{SystemTime, UNIX_EPOCH};

// use algorithms::search::linear_search;
use algorithms::sort::Sort;
use utils::ArrayGenerator;

use crate::utils::SortingHelper;

// use util::ArrayGenerator;

mod algorithms;
mod testing;
mod utils;

fn main() {

    let data_size = [10000, 100000];
    for n in data_size {
        let mut arr1 = ArrayGenerator::generate_random_array(n, n);
        let mut arr2 = ArrayGenerator::generate_random_array(n, n);

        println!("Random Array: ");
        SortingHelper::sort_test("insertion_sort", &mut arr1);
        SortingHelper::sort_test("selection_sort", &mut arr2);

        let mut arr1 = ArrayGenerator::generator_ordered_array(n);
        let mut arr2 = ArrayGenerator::generator_ordered_array(n);

        println!("Ordered Array: ");
        SortingHelper::sort_test("insertion_sort", &mut arr1);
        SortingHelper::sort_test("selection_sort", &mut arr2);
    }
    
}
