use std::path::PathBuf;

#[derive(Serialize)]
pub struct Entry {
    path: String,
    name: String,
    bsize: u64,
    hsize: f64,
    bstring: u64,
    acro_bstring: String,
    datetime: String,
    timestamp: i64,
    elements: u64,
}

impl Entry {

    pub fn new(p: &PathBuf) {
        Entry {
            path: super::get_path_string(p),
            name: super::get_filename(p),
            bsize: super::get_size(p),
            hsize: 0.0f64,
            datetime: super::get_datetime(p),
            timestamp: super::get_timestamp(p),
            elements: super::get_nb_elements(p),
        }
    }

    pub fn set_elements(&mut self, elements: u64) {
        self.elements = elements;
    }

}
