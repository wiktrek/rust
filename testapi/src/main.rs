use rocket::*;
mod routes;
use routes::*;
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index_rs, save, delete, date_rs, redirect_rs, google_rs])
}
