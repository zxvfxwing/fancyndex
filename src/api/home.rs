use std::path::PathBuf;
use filesystem::scanner::Scanner;

use rocket_contrib::{Json, Template};
use rocket::response::Redirect;

use filesystem::{
    pbuf_is_hidden, 
    pbuf_is_symlink,
    unsafepath::UnsafePBuf,
    Directory
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

#[get("/<unsafe_p..>")]
pub fn path(unsafe_p: UnsafePBuf) -> Result<Template, Redirect> {
    let pbuf = PathBuf::new()
        .join("/home/spoken")
        .join(unsafe_p.path());
    
    let scan = Scanner::new(&pbuf)
        .do_hidden(true)
        .do_symlink(true);

    Ok(Template::render(
        "index",
        scan.entries()
            .fix_url(
                &PathBuf::new().join("/home/spoken"), 
                &PathBuf::new().join("/home")
            )
    ))
}

#[get("/<unsafe_p..>")]
pub fn size(unsafe_p: UnsafePBuf) -> Result<Json<Directory>, Redirect> {
    let pbuf = PathBuf::new()
        .join("/home/spoken")
        .join(unsafe_p.path());

    println!("{}", pbuf.display());
    
    let scan = Scanner::new(&pbuf)
        .do_hidden(true)
        .do_symlink(true);
    
    Ok(Json(
        scan.deep_run()
            .fix_url(
                &PathBuf::new().join("/home/spoken"), 
                &PathBuf::new().join("/home")
            )
    ))
}