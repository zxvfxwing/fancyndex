use rocket::response::Redirect;

pub mod home;

/// Redirect to /home index when "/" is triggered
#[get("/")]
pub fn redirect_home() -> Redirect {
    Redirect::to("/home")
}



/* TEST

/*
* curl -H "Content-Type: application/json" -X POST -d '{"size":0,"elements":0,"directories":[{"path":"/home/spoken/Git/dotconfig","name":"dotconfig","size":0,"file":false,"elements":1},{"path":"/home/spoken/Git/M1","name":"M1","size":0,"file":false,"elements":1},{"path":"/home/spoken/Git/fancyndex","name":"fancyndex","size":0,"file":false,"elements":1}],"files":[]}' http://localhost:8000/test
*/

#[get("/")]
fn index() -> Json<Entries> {
    let p = filesystem::pbuf_cdir();
    let walker = Walker::new(&p, false, false);
    Json(walker.run())
}

#[post("/test", format = "application/json", data = "<entries>")]
fn test(mut entries: Json<Entries>, cfg: State<Config>) -> Json<Entries> {
    entries.process_deep_run(cfg.walk_opt.hidden, cfg.walk_opt.symlink);
    entries
}

*/

