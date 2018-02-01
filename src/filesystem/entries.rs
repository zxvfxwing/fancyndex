use walkdir::DirEntry;

pub struct Entry {
    pub absolute_path: String,
    pub name: String,
    pub size: u64,
    pub file: bool,
}

pub struct Entries {
    pub directories: Vec<Entry>,
    pub files: Vec<Entry>,
}

impl Entry {
    pub fn new(entry: &DirEntry) -> Entry {

        let mut file = true;
        if !entry.file_type().is_file() {
            file = false;
        }

        Entry {
            absolute_path: super::path_string( &entry.path().to_path_buf() ),
            name: {
                match super::get_file_name(entry) {
                    Ok(name) => name,
                    Err(_) => "".to_string(),
                }                
            },
            size: {
                if file {
                    super::get_file_size(entry)
                }
                else {
                    0u64
                }
            },
            file,
        }
    }

    pub fn is_file(&self) -> bool {
        self.file
    }
}

impl Entries {
    pub fn new() -> Entries {
        Entries{
            directories: Vec::new(),
            files: Vec::new(),
        }
    }

    pub fn push_el(&mut self, e: Entry) {
        if e.is_file() {
            self.files.push( e );
        }
        else {
            self.directories.push( e );
        }
    }

    pub fn nb_elts(&self) -> usize {
        self.directories.len() + self.files.len()
    }
}