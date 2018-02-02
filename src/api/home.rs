use rocket::State;
use rocket_contrib::Template;
use rocket::response::Redirect;

use config::Config;
use walker::Walker;
use filesystem::{pbuf_str, pbuf_is_dir};
use filesystem::unsafepath::UnsafePBuf;
use std::path::PathBuf;

/* Add also a Result and a Redirect to a Page Error 
*  Error can only occurs if user didn't call `check()` function on Config object.
*/
#[get("/")]
pub fn index(cfg: State<Config>) -> Template {
    let h_path = PathBuf::new().join(&cfg.root.path); /* Home Path */
    let walker = Walker::new(&h_path, cfg.walk_opt.hidden, cfg.walk_opt.symlink);
    let entries = walker.run().toggle_prefix(&cfg.root.path, &PathBuf::new().join("/home"));
    Template::render("index", entries)
}

#[get("/<unsafe_p..>")]
pub fn path(cfg: State<Config>, unsafe_p: UnsafePBuf) -> Result<Template, Redirect> {
    let c_path = PathBuf::new().join(&cfg.root.path).join(unsafe_p.path()); /* Current Path */

    if !pbuf_is_dir(&c_path) {
        let mut r_path = PathBuf::new().join("/home").join(&unsafe_p.path()); /* Current Path URL */
        r_path.pop(); /* path's parent */
        return Err(Redirect::to(pbuf_str(&r_path)));
    }

    let walker = Walker::new(&c_path, cfg.walk_opt.hidden, cfg.walk_opt.symlink);
    let entries = walker.run().toggle_prefix(&cfg.root.path, &PathBuf::new().join("/home"));
    Ok(Template::render("index", entries))
}