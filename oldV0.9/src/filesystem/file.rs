use std::path::PathBuf;

#[derive(Serialize)]
pub struct File {
    name: String,
    size: u64,
    hsize: f64,
    unit: String,
    short_unit: String,
    timestamp: i64,
    datetime: String,
}

impl File {
    pub fn new(p: &PathBuf, size: u64, mode: bool) -> File {
        let hsize = super::get_human_size(size, mode);

        File {
            name: super::get_filename(p),
            size: size,
            hsize: hsize.0,
            unit: hsize.1,
            short_unit: hsize.2,
            timestamp: super::get_timestamp(p),
            datetime: super::get_datetime(p),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_size(&self) -> u64  {
        self.size
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }
}
