use actix_web::{web};

use crate::app::controllers::*;

// build application routes
pub fn build_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // routes
        .service(
            web::resource("/auth")
                .route(web::post().to(auth_controller::login))
                .route(web::delete().to(auth_controller::logout))
                .route(web::get().to(auth_controller::get_me))
        )
        .service(
            web::resource("/word_entries")
                .route(web::get().to(word_entries_controller::list_word_entries))
                // .route(web::post().to(word_entries_controller::create_word_entry))
        );
}
