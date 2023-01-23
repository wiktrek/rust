#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::response::Redirect;
use rocket::http::uri::Origin;
use rocket::serde::json::{json, Value};
const SEARCH_PREFIX: Origin<'static> = uri!("/search");
#[get("/")]
fn index() -> Redirect {
    let msg:Option<&str> = None;
    Redirect::to(uri!(SEARCH_PREFIX, search(msg)))
}

#[get("/search?<msg>")]
fn search(msg: Option<&str>) -> Value {
// Status::NoContent;
// if let Some(msg) = msg {
//     return Status::NoContent;
// }
json!({"notes": "It works"})
}
#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/",routes![index])
    .mount(SEARCH_PREFIX,routes![search])
}