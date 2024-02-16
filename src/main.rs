use data_structure::linear_search;
use testing::Student;

pub mod data_structure;
pub mod testing;

fn main() {
    let data = [25, 22, 123, 4214, 34523, 12, 321, 2];
    let res = linear_search::search(&data, 10);

    println!("linearSearch res is {res}");

    let student = [
        Student {
            name: "xing".to_string(),
        },
        Student {
            name: "guang".to_string(),
        },
    ];

    let xing = Student {
        name: "Xing".to_string(),
    };

    let res1 = linear_search::search(&student, xing);

    println!("{res1}");
}
