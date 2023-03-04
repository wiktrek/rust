use rocket::*;
mod routes;
use routes::date::date;
use routes::hello::hello;
use routes::index::index;
use routes::file::save;
use routes::file::delete;
use routes::redirect::redirect;
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, index, save, delete, date, redirect])
}
