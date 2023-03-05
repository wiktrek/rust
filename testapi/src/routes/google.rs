use rocket::*;
use rocket::response::Redirect;
#[get("/google?<query>")]
pub fn redirect(query: &str) -> Redirect {
    let go = format!("https://www.google.com/search?q={}", query);
    println!("{}, {}", go, query);
    Redirect::to(go)
}