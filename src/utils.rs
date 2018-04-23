use std::path::PathBuf;

pub fn toggle_prefix(path: &PathBuf, o_prefix: &PathBuf, n_prefix: &PathBuf) -> PathBuf {
    let stripped = path.strip_prefix(o_prefix).unwrap();
    let pbuf: PathBuf;

    /* a e s t h e t i c */
    if stripped.to_str().unwrap() != "" {
        pbuf = n_prefix.join(&stripped);
    }
    else {
        pbuf = n_prefix.to_owned();
    }

    pbuf
}