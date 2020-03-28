#[macro_use]
extern crate diesel;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod database;
mod errors;
mod models;
mod schema;
mod security;

mod auth_handler;
mod register_handler;

/// Main application entry
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,diesel=debug");
    env_logger::init();
    dotenv::dotenv().ok();

    // set up database connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    println!("db: {}", connspec);
    
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // app domain 
    let domain: String =
        std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    

    let bind = "127.0.0.1:8080";

    println!("Starting server at: {}", &bind);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            // enable logger
            .wrap(middleware::Logger::default())
            // identity
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(security::SECRET_KEY.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(domain.as_str())
                    .max_age_time(chrono::Duration::days(1))
                    .secure(false), // this can only be true if you have https
            ))
            // json request parsing config
            .data(web::JsonConfig::default().limit(4096))

            // routes
            .service(
                web::resource("/register")
                    .route(web::post().to(register_handler::register_user)),
            )
            .service(
                web::resource("/auth")
                    .route(web::post().to(auth_handler::login))
                    .route(web::delete().to(auth_handler::logout))
                    .route(web::get().to(auth_handler::get_me)),
            )
    })
    .bind(&bind)?
    .run()
    .await
}
