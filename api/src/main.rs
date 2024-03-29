use actix_web::{get, http::StatusCode, web, App, HttpResponse, HttpServer, Result};
use open::that;

#[get("/hi/{name}")] // <- define path parameters
async fn hi(path: web::Path<String>) -> Result<String> {
    let name = path.into_inner();
    Ok(format!("Hi {}", name))
}

#[get("/")]
async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("index.html")))
}

fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(hi);
}

// this function could be located in a different module
fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");
    let _ = that("http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .configure(config)
            .service(web::scope("/api").configure(scoped_config))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
