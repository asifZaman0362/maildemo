use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use actix_files::NamedFile;
use actix_files::Files;
mod encryption;
mod database;
mod types;

#[post("/login")]
fn login() -> impl Responder {
    
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| 
            App::new()
                .service(Files::new("/", "../public/").index_file("login.html")))
        .bind(("localhost", 8080))?
        .run().await
}
