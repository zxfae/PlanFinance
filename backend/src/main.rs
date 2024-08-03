mod errors;
use std::fs::File;
use std::io::Read;
use actix_files as fs;
use actix_web::{get, App, HttpServer, HttpResponse, Result};
use actix_web::middleware::Logger;
use errors::Error;

fn read_file(path: &str) -> Result<String, Error> {
    let mut file = File::open(path).map_err(|_| Error::FileNotFound)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|_| Error::InternalServerError)?;
    Ok(contents)
}

#[get("/")]
async fn index() -> Result<HttpResponse, Error> {
    let html = read_file("./frontend/static/index.html")?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

#[get("/success")]
async fn success() -> Result<HttpResponse, Error> {
    let html = read_file("./frontend/static/index.html")?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

#[get("/successed")]
async fn successed() -> Result<HttpResponse, Error> {
    let html = read_file("./frontend/static/index.html")?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

#[get("/recapone")]
async fn recapone() -> Result<HttpResponse, Error> {
    let html = read_file("./frontend/static/index.html")?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(index)
            .service(success)
            .service(successed)
            .service(recapone)
            .service(fs::Files::new("/static/pkg", "./frontend/pkg").show_files_listing())
            .service(fs::Files::new("/static", "./frontend/static").show_files_listing())
    })
        .bind(("127.0.0.1", 8000))?
        .keep_alive(std::time::Duration::from_secs(75))
        .run()
        .await
}
