use rocket::*;
use chrono::prelude::*;
#[get("/date")]
pub fn date() -> String {
    let local_time = Local::now();
    format!("{}", local_time)
}