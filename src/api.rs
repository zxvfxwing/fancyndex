use std::path::PathBuf;
use rocket_contrib::Json;
use filesystem;
use filesystem::directory;

#[derive(Serialize)]
struct EntryJSON {
    name: String,
    size: u64,
    timestamp: i64,
    datetime: String,
    elements: u64
}

#[derive(Serialize)]
pub struct ApiJSON {
    name: String,
    size: u64,
    timestamp: i64,
    datetime: String,
    elements: u64,
    total_files: u64,
    directories: Vec<EntryJSON>,
    files: Vec<EntryJSON>
}

impl ApiJSON {

    fn new(p: &PathBuf) -> ApiJSON {
        println!("{}", p.display());
        let dir = directory::Directory::new(p);

        ApiJSON {
            name: {
                let home = filesystem::get_parent_current_dir();
                if filesystem::get_filename(&home) == dir.name() {
                    "Home".to_string()
                }
                else { dir.name() }
            },
            size: dir.size(),
            timestamp: dir.timestamp(),
            datetime: dir.datetime(),
            elements: dir.nb_elements(),
            total_files: dir.nb_total_files(),

            directories: {
                let mut directories = Vec::new();
                for d in dir.directories() {
                    let dir_json = EntryJSON {
                        name: d.name(),
                        size: d.size(),
                        timestamp: d.timestamp(),
                        datetime: d.datetime(),
                        elements: d.nb_elements(),
                    };
                    directories.push(dir_json);
                }
                directories
            },

            files: {
                let mut files = Vec::new();
                for f in dir.files() {
                    let file_json = EntryJSON {
                        name: f.name(),
                        size: f.size(),
                        timestamp: f.timestamp(),
                        datetime: f.datetime(),
                        elements: 0,
                    };
                    files.push(file_json);
                }
                files
            }
        }
    }

    pub fn list_dir(p: &PathBuf) -> Json<ApiJSON> {
        Json(ApiJSON::new(p))
    }
}
