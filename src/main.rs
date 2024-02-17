use std::time::{SystemTime, UNIX_EPOCH};

use data_structure::linear_search;
use util::ArrayGenerator;

pub mod data_structure;
pub mod testing;
pub mod util;

fn main() {
    let data_size = [1000000, 10000000];

    for n in data_size {
        let data = ArrayGenerator::generator_ordered_array(n);
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get timestamp")
            .as_secs_f64();
        for _ in 0..100 {
            linear_search::search(&data, n);
        }
        let end_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get timestamp")
            .as_secs_f64();

        let time = end_time - start_time;
        println!("n = {n}, 100 runs: {time} s")
    }
}
