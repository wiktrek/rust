use rocket::*;
mod routes;


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![routes::hello::hello, routes::index::index, routes::file::save, routes::file::delete, routes::date::date, routes::redirect::redirect])
}
