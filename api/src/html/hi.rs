use actix_web::{ get, web,Result };

#[get("/hi/{name}")] // <- define path parameters
async fn hi(path: web::Path<String>) -> Result<String> {
    let name = path.into_inner();
    Ok(format!("Hi {}", name))
}