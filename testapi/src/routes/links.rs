use rocket::*;
use rocket::response::Redirect;
#[get("/yt")]
pub fn yt() -> Redirect{
    let go = "https://wiktrek.xyz/yt";
    Redirect::to(go)
}
#[get("/ig")]
pub fn ig() -> Redirect{
    let go = "https://wiktrek.xyz/ig";
    Redirect::to(go)
}
#[get("/gh")]
pub fn gh() -> Redirect{
    let go = "https://wiktrek.xyz/gh";
    Redirect::to(go)
}
#[get("/ttv")]
pub fn ttv() -> Redirect{
    let go = "https://wiktrek.xyz/ttv";
    Redirect::to(go)
}