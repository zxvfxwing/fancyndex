use rocket::State;
use rocket_contrib::Template;
use rocket::response::Redirect;

use config::Config;
use walker::Walker;
use filesystem::{pbuf_is_dir, pbuf_parent_cdir};
use filesystem::unsafepath::UnsafePBuf;

#[get("/")]
pub fn index(cfg: State<Config>) -> Template {
    let home_path = pbuf_parent_cdir();
    let walker = Walker::new(&home_path, cfg.walk_opt.hidden, cfg.walk_opt.symlink);
    Template::render("index", walker.run())
}

#[get("/<unsafe_p..>")]
pub fn path(cfg: State<Config>, unsafe_p: UnsafePBuf) -> Result<Template, Redirect> {
    let c_path = pbuf_parent_cdir().join(unsafe_p.path());

    if !pbuf_is_dir(&c_path){
        return Err(Redirect::to("/home"));
    }

    let walker = Walker::new(&c_path, cfg.walk_opt.hidden, cfg.walk_opt.symlink);
    Ok(Template::render("index", walker.run()))
}