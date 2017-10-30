pub struct File {
    pub name: String,
}

impl File {
    pub fn name(&self) -> &str {
        &self.name
    }
}
