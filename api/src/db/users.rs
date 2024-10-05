use actix_web::{ get, http::StatusCode, web, App, HttpResponse, HttpServer, Result };
#[get("/db/users")]
pub async fn db_users() -> Result<String> {    
    Ok(format!("Users"))
}