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

#[get("/")]
pub fn index(cfg: State<Config>) -> Result<Template, Redirect> {
    let h_path = PathBuf::new().join(&cfg.root.path);

    if rules_check(&h_path, &cfg) {
        return Err(Redirect::to("/error/config"))
    }

    let walkdir = WalkDirBuilder::new(h_path)
                                    .do_hidden(cfg.walk_opt.hidden)
                                    .do_symlink(cfg.walk_opt.symlink)
                                    .use_entries_opt(cfg.entries_opt.clone())
                                    .build();

    match walkdir.scan() {
        Some(mut entries) => {
            entries.toggle_prefix(&cfg.root.path, &PathBuf::new().join("/home"));
            Ok(Template::render("index", entries))
        },
        None => Err(Redirect::to("/error/read"))
    }
}

#[get("/<unsafe_p..>")]
pub fn path(cfg: State<Config>, unsafe_p: UnsafePBuf) -> Result<Template, Redirect> {
    let current_path = PathBuf::new()
                                .join(&cfg.root.path)
                                .join(unsafe_p.path());

    let home = PathBuf::new().join("/home");

    let mut fail_url = home.join(&unsafe_p.path());
    if rules_check(&current_path, &cfg) {
        fail_url.pop();
        return Err(Redirect::to(pbuf_str(&fail_url)))
    }

    let walkdir = WalkDirBuilder::new(current_path)
                                .do_hidden(cfg.walk_opt.hidden)
                                .do_symlink(cfg.walk_opt.symlink)
                                .use_entries_opt(cfg.entries_opt.clone())
                                .build();
                        
    match walkdir.scan() {
        Some(mut entries) => {
            entries.toggle_prefix(&cfg.root.path, &home);
            Ok(Template::render("index", entries))
        },
        None => Err(Redirect::to("/error/read"))
    }
}