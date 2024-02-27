use std::fmt;

pub struct Student {
    pub name: String,
    pub score: isize,
}

impl PartialEq for Student {
    fn eq(&self, other: &Self) -> bool {
        self.name.to_lowercase() == other.name.to_lowercase()
    }
}

impl PartialOrd for Student {
    fn ge(&self, other: &Self) -> bool {
        if self.score >= other.score {
            return true;
        }
        false
    }
    // fn gt(&self, other: &Self) -> bool {
    //     if self.score > other.score {
    //         return true;
    //     }
    //     false
    // }
    // fn le(&self, other: &Self) -> bool {
    //     if self.score <= other.score {
    //         return false;
    //     }
    //     true
    // }
    // fn lt(&self, other: &Self) -> bool {
    //     if self.score < other.score {
    //         return false;
    //     }
    //     true
    // }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Student(name: {}, score: {})", self.name, self.score)
    }
}
