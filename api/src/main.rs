use actix_web::{ web, App, HttpResponse, HttpServer };
use open::that;
mod func;
mod db;
mod html;
use func::*;
use db::*;
use html::*;
fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service((hi, db_users));
}

// this function could be located in a different module
fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed))
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let user = connect().await.unwrap();
    println!("Starting server at http://127.0.0.1:8080");
    let _ = that("http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .configure(config)
            .service(web::scope("/api").configure(scoped_config))
            .service(index)
    })
        .bind(("127.0.0.1", 8080))?
        .run().await
}