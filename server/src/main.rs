use actix_session::storage::CookieSessionStore;
use actix_web::cookie::Key;
use actix_web::error::ErrorUnavailableForLegalReasons;
use actix_web::http::header::{CONTENT_TYPE, ContentType, LOCATION};
use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse, HttpRequest, Result};
use actix_files::NamedFile;
use actix_files::Files;
use actix_session::{Session, SessionMiddleware};

mod encryption;
mod database;
mod types;

use types::{LoginData, AppState};
use crate::encryption::auth;


#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open_async("../public/login.html").await
}

async fn post_login(state: web::Data<AppState>, form_data: web::Form<LoginData>, session: Session) -> Result<HttpResponse, actix_web::Error> {
    let username = &form_data.email_address;
    let password = &form_data.password;
    println!("request login: username: {}, password: {}", username, password);
    let mut conn = &mut *state.database.lock().unwrap();
    let result = auth::login(username.as_str(), password.as_str(), &mut conn, &session);
    match result {
        Ok(()) => {
            println!("Logged in!");
            Ok(HttpResponse::Found().insert_header((LOCATION, "/")).finish())
        }
        Err(err) => {
            println!("Failed to log in!");
            Err(ErrorUnavailableForLegalReasons(err))
        }
    }
}

async fn login(req: HttpRequest) -> impl Responder {
    NamedFile::open_async("../public/login.html").await
}

fn hash() {
    if let Some(hash) = encryption::hasher::compute_hash("lmao") {
        println!("hash: {}", hash);
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    hash();
    HttpServer::new(|| 
            App::new()
                .wrap(
                    SessionMiddleware::new(
                        CookieSessionStore::default(),
                        Key::generate()
                    )
                )
                .app_data(web::Data::new(AppState::new("data.db")))
                //.service(web::resource("/login").to(login))
                .service(web::resource("/login").route(web::post().to(post_login)))
                .service(Files::new("/", "../public/").index_file("login.html")))
        .bind(("localhost", 8080))?
        .run().await
}
