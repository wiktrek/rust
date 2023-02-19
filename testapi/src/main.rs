use rocket::*;
mod routes;


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![routes::hello::hello, routes::index::index, routes::test::save, routes::test::delete])
}
