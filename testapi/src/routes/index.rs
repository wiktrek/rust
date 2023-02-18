use rocket::*;
use rocket::response::content::RawHtml;
#[get("/")]
pub fn index() -> RawHtml<&'static str> {
    RawHtml(r#"
    <a href="https://wiktrek.xyz">wiktrek.xyz</a> 
    "#)
}