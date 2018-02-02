use rocket::State;
use rocket_contrib::Template;
use rocket::response::Redirect;

use config::Config;
use walker::Walker;
use filesystem::{pbuf, pbuf_str, pbuf_is_dir, pbuf_from_string, pbuf_parent};
use filesystem::unsafepath::UnsafePBuf;

#[get("/")]
pub fn index(cfg: State<Config>) -> Template {
    let h_path = pbuf_from_string(&cfg.root.path); /* Home Path */
    let walker = Walker::new(&h_path, cfg.walk_opt.hidden, cfg.walk_opt.symlink);
    Template::render("index", walker.run())
}

#[get("/<unsafe_p..>")]
pub fn path(cfg: State<Config>, unsafe_p: UnsafePBuf) -> Result<Template, Redirect> {
    let c_path = pbuf_from_string(&cfg.root.path).join(unsafe_p.path()); /* Current Path */

    if !pbuf_is_dir(&c_path){
        let r_path = pbuf().join("/home").join(&unsafe_p.path());
        return Err(Redirect::to( pbuf_str(&pbuf_parent(&r_path)) ));
    }

    let walker = Walker::new(&c_path, cfg.walk_opt.hidden, cfg.walk_opt.symlink);
    Ok(Template::render("index", walker.run()))
}