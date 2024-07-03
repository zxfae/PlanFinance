use actix_files as fs;
use actix_web::{get, App, HttpServer, HttpResponse, Result};
use reqwest;

#[get("/")]
async fn index() -> Result<HttpResponse> {
    let html = std::fs::read_to_string("./frontend/static/index.html")?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

#[get("/users")]
async fn get_users() -> Result<HttpResponse> {
    let response = reqwest::get("http://localhost:8080/users")
        .await
        .expect("Failed to send request");

    let body = response.text().await.expect("Failed to read response body");
    Ok(HttpResponse::Ok().body(body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(get_users)
            .service(fs::Files::new("/static/pkg", "./frontend/pkg").show_files_listing())
            .service(fs::Files::new("/static", "./frontend/static").show_files_listing())
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
