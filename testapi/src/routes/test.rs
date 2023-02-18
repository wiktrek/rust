use rocket::*;
#[get("/save/<user>/<data>")]
pub fn test(user: String, data: String) -> String{
    format!("data:{data}, user: {user}")
}