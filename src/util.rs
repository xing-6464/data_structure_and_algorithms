pub struct ArrayGenerator;

impl ArrayGenerator {
    pub fn generator_ordered_array(n: isize) -> Vec<isize> {
        let mut res_array = Vec::new();

        for i in 0..n {
            res_array.push(i);
        }

        res_array
    }
}
