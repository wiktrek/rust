use rocket::*;
use rocket::response::Redirect;
#[get("/redirect/<link>")]
pub fn redirect(link: &str) -> Redirect {
    let go = format!("https://wiktrek.xyz/go/{}", link);
    println!("{}, {}", go, link);
    Redirect::to(go)
}