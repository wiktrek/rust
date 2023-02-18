use rocket::*;
#[get("/save/<user>/<data>")]
pub fn db(user: String, data: String) -> String{
    format!("data:{data}, user: {user}")
}