use data_structure::linear_search;

pub mod data_structure;

fn main() {
    let data = [25, 22, 123, 4214, 34523, 12, 321, 2];
    let res = linear_search::search(&data, 10);

    println!("linearSearch res is {res}");
}
