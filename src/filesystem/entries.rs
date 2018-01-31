pub struct Entry {
    name: String,
    size: u64,
}

pub struct Entries {
    directories: Vec<Entry>,
    files: Vec<Entry>,
}

impl Entry {
    pub fn new(name: String, size: u64) -> Entry {
        Entry {
            name,
            size,
        }
    }
}

impl Entries {
    pub fn new() -> Entries {
        Entries{
            directories: Vec::new(),
            files: Vec::new(),
        }
    }

    pub fn push_dir(&mut self, dir: Entry) {
        self.directories.push(dir)
    }

    pub fn push_file(&mut self, file: Entry) {
        self.files.push(file)
    }

    pub fn nb_elts(&self) -> usize {
        self.directories.len() + self.files.len()
    }
}