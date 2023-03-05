use rocket::*;
use rocket::response::Redirect;
#[get("/yt")]
pub fn yt() -> Redirect{
    let go = "https://wiktrek.xyz/yt";
    Redirect::to(go)
}
