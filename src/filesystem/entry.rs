use std::path::PathBuf;

#[derive(Serialize)]
pub struct Entry {
    name: String,
    size: u64,
    datetime: String,
    timestamp: i64,
    elements: u64,
}

impl Entry {

    pub fn new(p: &PathBuf) -> Entry {
        Entry {
            name: super::get_filename(p),
            size: super::get_size(p),
            datetime: super::get_datetime(p),
            timestamp: super::get_timestamp(p),
            elements: 0u64,
        }
    }

    pub fn set_elements(&mut self, elements: u64) {
        self.elements = elements;
    }

}
