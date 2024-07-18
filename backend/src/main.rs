use std::time::Duration;
use actix_files as fs;
use actix_web::{get, App, HttpServer, HttpResponse, Result, Responder};
use reqwest;

#[get("/")]
async fn index() -> Result<HttpResponse> {
    let html = std::fs::read_to_string("./frontend/static/index.html")?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

#[get("/users")]
async fn get_users() -> Result<HttpResponse> {
    let response = reqwest::get("http://localhost:8080/get_user")
        .await
        .expect("Failed to send request");
    let body = response.text().await.expect("Failed to read response body");
    Ok(HttpResponse::Ok().body(body))
}

#[get("/success")]
async fn success() -> impl Responder {
    let html = std::fs::read_to_string("./frontend/static/index.html").unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}

#[get("/successed")]
async fn successed() -> impl Responder {
    let html = std::fs::read_to_string("./frontend/static/index.html").unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}

#[get("/planfinancement")]
async fn planfinancement() -> impl Responder {
    let html = std::fs::read_to_string("./frontend/static/index.html").unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(success)
            .service(successed)
            .service(planfinancement)
            .service(get_users)
            .service(fs::Files::new("/static/pkg", "./frontend/pkg").show_files_listing())
            .service(fs::Files::new("/static", "./frontend/static").show_files_listing())
    })
        .bind(("127.0.0.1", 8000))?
        //Performance and ressource management
        .keep_alive(Duration::from_secs(75))
        .run()
        .await
}
