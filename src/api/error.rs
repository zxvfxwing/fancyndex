use rocket::State;
use rocket_contrib::Template;
use rocket::response::Redirect;

#[get("/config/fail")]
pub fn config_fail() -> &'static str {
    "Error: something went terrible wrong ...\n
You should check your Fancyndex.toml\n
Info: root.path not OK.\n
You should use `check()` function on Config::new()."
}