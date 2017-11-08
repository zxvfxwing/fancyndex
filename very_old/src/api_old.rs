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
pub struct FullJSON {
    name: String,
    size: u64,
    timestamp: i64,
    datetime: String,
    elements: u64,
    total_elements: u64,
    directories: Vec<EntryJSON>,
    files: Vec<EntryJSON>
}

#[derive(Serialize)]
pub struct DirJSON {
    name: String,
    timestamp: i64,
    datetime: String,
    elements: u64,
}


pub fn full(p: &PathBuf) -> FullJSON {
    let dir = directory::Directory::new(p, -1, 0);

    FullJSON {
        name: {
            /* Créer une fonction pour check le nom d'un fichier */
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
        total_elements: dir.nb_total_elements(),

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

pub fn dir_light_info(p: &PathBuf) -> Json<DirJSON> {
    let dir = directory::Directory::soft_new(p);

    let dir_json = DirJSON {
        name: dir.name(),
        timestamp: dir.timestamp(),
        datetime: dir.datetime(),
        elements: {
            match dir.path().read_dir() {
                Ok(entries) => entries.count() as u64,
                Err(_) => 0u64,
            }
        }
    };

    Json(dir_json)
}

pub fn list_full_dir(p: &PathBuf) -> Json<FullJSON> {
    Json(full(p))
}
