use rocket::*;
use rocket::http::uri::Absolute;
#[get("/redirect/<link>")]
pub fn redirect(link: &str) -> String {
    let link = format!("https://wiktrek.xyz/go/{}", link);
    response::Redirect::temporary(uri!("https://wiktrek.xyz/"));
    format!("{}", link)
}