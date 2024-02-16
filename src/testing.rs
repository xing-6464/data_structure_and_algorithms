pub struct Student {
    pub name: String,
}

impl PartialEq for Student {
    fn eq(&self, other: &Self) -> bool {
        self.name.to_lowercase() == other.name.to_lowercase()
    }
}
