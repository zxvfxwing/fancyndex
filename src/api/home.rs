use rocket::State;
use rocket_contrib::Template;
use rocket::response::Redirect;

use config::Config;

use filesystem::walkdir::WalkDirBuilder;

use filesystem::{pbuf_str, pbuf_is_dir, pbuf_is_hidden, pbuf_is_symlink};
use filesystem::unsafepath::UnsafePBuf;
use std::path::PathBuf;

fn rules_check(p: &PathBuf, cfg: &State<Config>) -> bool {
    if !pbuf_is_dir(&p) {
        return true
    }
    else 
    if pbuf_is_hidden(&p) && !cfg.walk_opt.hidden {
        return true
    }
    else
    if pbuf_is_symlink(&p) && !cfg.walk_opt.symlink {
        return true
    }

    false
}

/* TODO: 
*  Add also a Result and a Redirect to a Page Error 
*  Error can only occurs if user didn't call `check()` function on Config object.
*/
#[get("/")]
pub fn index(cfg: State<Config>) -> Result<Template, Redirect> {
    let h_path = PathBuf::new().join(&cfg.root.path); /* Home Path */
    let fail_url = PathBuf::new().join("/error/config/fail");

    if rules_check(&h_path, &cfg) {
        return Err(Redirect::to(pbuf_str(&fail_url)))
    }

    let walkdir = WalkDirBuilder::new(h_path)
                                    .do_hidden(cfg.walk_opt.hidden)
                                    .do_symlink(cfg.walk_opt.symlink)
                                    .use_entries_opt(cfg.entries_opt.clone())
                                    .build();
                        
    Ok(Template::render("index", walkdir.scan().unwrap()))
}

#[get("/<unsafe_p..>")]
pub fn path(cfg: State<Config>, unsafe_p: UnsafePBuf) -> Result<Template, Redirect> {
    let c_path = PathBuf::new().join(&cfg.root.path).join(unsafe_p.path()); /* Current Path */

    let url_home = PathBuf::new().join("/home");

    let mut url = url_home.join(&unsafe_p.path());
    if rules_check(&c_path, &cfg) {
        url.pop();
        return Err(Redirect::to(pbuf_str(&url)))
    }

    let walkdir = WalkDirBuilder::new(c_path)
                                    .do_hidden(cfg.walk_opt.hidden)
                                    .do_symlink(cfg.walk_opt.symlink)
                                    .use_entries_opt(cfg.entries_opt.clone())
                                    .build();
                        
    Ok(Template::render("index", walkdir.scan().unwrap()))
}