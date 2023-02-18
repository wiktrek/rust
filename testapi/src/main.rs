use rocket::*;
use rocket::response::content::RawHtml;
mod routes;
#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(r#"
    <a href="https://wiktrek.xyz">wiktrek.xyz</a> 
    "#)
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![routes::hello::hello, index])
}
