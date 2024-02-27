use std::time::{SystemTime, UNIX_EPOCH};

// use algorithms::search::linear_search;
use algorithms::sort;

// use util::ArrayGenerator;

pub mod algorithms;
pub mod testing;
pub mod util;

fn main() {
    // let data_size = [1000000, 10000000];

    // for n in data_size {
    //     let data = ArrayGenerator::generator_ordered_array(n);
    //     let start_time = SystemTime::now()
    //         .duration_since(UNIX_EPOCH)
    //         .expect("Failed to get timestamp")
    //         .as_secs_f64();
    //     for _ in 0..100 {
    //         linear_search::search(&data, n);
    //     }
    //     let end_time = SystemTime::now()
    //         .duration_since(UNIX_EPOCH)
    //         .expect("Failed to get timestamp")
    //         .as_secs_f64();

    //     let time = end_time - start_time;
    //     println!("n = {n}, 100 runs: {time} s")
    // }

    let mut arr = [1, 4, 2, 3, 6, 5];

    sort::selection_sort(&mut arr);
    for (i, v) in arr.iter().enumerate() {
        println!("{i} : {v}");
    }

    let mut student1 = [
        testing::Student {
            name: String::from("guang"),
            score: 100,
        },
        testing::Student {
            name: String::from("xing"),
            score: 90,
        },
        testing::Student {
            name: String::from("Bobo"),
            score: 80,
        },
    ];
    sort::selection_sort(&mut student1);
    for student in student1 {
        println!("{student} ")
    }
}
