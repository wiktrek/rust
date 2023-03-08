use rocket::*;
use rocket::response::content::RawHtml;
#[get("/")]
pub fn index() -> RawHtml<&'static str> {
    RawHtml(include_str!("../html/index.html"))
}