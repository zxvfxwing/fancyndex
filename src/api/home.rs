use std::path::PathBuf;
use filesystem::scanner::Scanner;

use rocket_contrib::Template;
use rocket::response::Redirect;

use filesystem::{
    pbuf_is_hidden, 
    pbuf_is_symlink,
    unsafepath::UnsafePBuf
};

#[get("/")]
pub fn index() -> Result<Template, Redirect> {
    let pbuf = PathBuf::new().join("/home/spoken");

    let scan = Scanner::new(&pbuf)
        .do_hidden(true)
        .do_symlink(true);


    Ok(Template::render(
        "index",
        scan.entries()
            .fix_url(&pbuf, &PathBuf::new().join("/home")) 
    ))
}