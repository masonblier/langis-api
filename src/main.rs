#![feature(trait_alias)]

#[macro_use]
extern crate diesel;

use actix_web::{middleware, web, App, HttpServer};

mod app;
mod schema;

#[cfg(test)]
mod tests;

use crate::app::config;
use crate::app::database::get_database_pool;
use crate::app::identity::get_identity_service;
use crate::app::routes::build_routes;

/// main
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,diesel=debug");
    env_logger::init();
    dotenv::dotenv().ok();

    // create shared database pool
    let pool = get_database_pool();

    // Start HTTP server
    println!("Starting server at: {}", config::BIND_ADDRESS.as_str());
    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            // enable logger
            .wrap(middleware::Logger::default())
            // identity
            .wrap(get_identity_service())
            // json request parsing config
            .data(web::JsonConfig::default().limit(4096))
            .configure(build_routes)
    })
    .bind(config::BIND_ADDRESS.as_str())?
    .run()
    .await
}