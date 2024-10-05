use actix_web::{ get, http::StatusCode, HttpResponse,Result };

#[get("/")]
pub async fn index() -> Result<HttpResponse> {
    Ok(
        HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(include_str!("index.html"))
    )
}

