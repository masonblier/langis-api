use actix_web::{web};

use crate::app::controllers::*;

// build application routes
pub fn build_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // routes
        .service(
            web::resource("/word_entries")
                .route(web::get().to(word_entries_controller::list_word_entries))
                // .route(web::post().to(word_entries_controller::create_word_entry))
        );
}