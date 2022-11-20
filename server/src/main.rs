use actix_session::storage::CookieSessionStore;
use actix_web::cookie::Key;
use actix_web::error::ErrorUnavailableForLegalReasons;
use actix_web::http::header::{CONTENT_TYPE, ContentType, LOCATION, self};
use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse, HttpRequest, Result};
use actix_files::NamedFile;
use actix_files::Files;
use actix_session::{Session, SessionMiddleware};

use std::io::Read;
use std::path::Path;

mod encryption;
mod database;
mod types;

use types::{LoginData, AppState};
use crate::encryption::auth;

fn redirect(target: &str) -> Result<HttpResponse> {
    Ok(HttpResponse::Found().insert_header((LOCATION, target)).finish())
}

async fn send_file<T>(filename: T) -> Result<HttpResponse>
where T: AsRef<Path>
{
    let file = NamedFile::open_async(filename).await?;
    let mut data = String::new();
    file.file().read_to_string(&mut data)?;
    Ok(HttpResponse::Ok()
        .insert_header(header::ContentType::html())
        .body(data))
}

async fn index(session: Session) -> impl Responder {
    println!("At index, Logging in as: {:?}", session.get::<String>("username"));
    if let Ok(username) = session.get::<String>("username") {
        if let Some(_) = username {
            send_file("../public/inbox.html").await
        } else {
            redirect("/login")
        }
    } else {
        redirect("/login")
    }
}

async fn compose(session: Session) -> impl Responder {
    println!("At compose, Logging in as: {:?}", session.get::<String>("username"));
    if let Ok(username) = session.get::<String>("username") {
        if let Some(user) = username {
            return send_file("../public/composer.html").await;
        }
    }
    redirect("/login")
}

async fn post_login(state: web::Data<AppState>, form_data: web::Form<LoginData>, session: Session) -> Result<HttpResponse, actix_web::Error> {
    println!("Logging in as: {:?}", session.get::<String>("username"));
    let username = &form_data.email_address;
    let password = &form_data.password;
    println!("request login: username: {}, password: {}", username, password);
    let mut conn = &mut *state.database.lock().unwrap();
    let result = auth::login(username.as_str(), password.as_str(), &mut conn, &session);
    match result {
        Ok(()) => {
            println!("Logged in!");
            Ok(HttpResponse::Found().insert_header((LOCATION, "/home")).finish())
        }
        Err(err) => {
            println!("Failed to log in!");
            Err(ErrorUnavailableForLegalReasons(err))
        }
    }
}

async fn login() -> impl Responder {
    send_file("../public/login.html").await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let key = Key::generate();
    HttpServer::new(move || 
            App::new()
                .wrap(
                    SessionMiddleware::new(
                        CookieSessionStore::default(),
                        key.to_owned()
                    )
                )
                .app_data(web::Data::new(AppState::new("data.db")))
                .service(web::resource("/login").route(web::post().to(post_login)))
                .service(web::resource("/login").route(web::get().to(login)))
                .service(web::resource("/compose").route(web::get().to(compose)))
                .service(web::resource("/home").route(web::get().to(index)))
                .service(web::resource("/").route(web::get().to(index)))
                .service(Files::new("/res/", "../public/res")))
        .bind(("localhost", 8080))?
        .run().await
}
