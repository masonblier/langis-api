use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize,Serialize};

use crate::app::database::DbPool;
use crate::app::errors::ServiceError;
use crate::app::models::{WordEntry};


/// GET /word_entries list endpoint
pub async fn list_word_entries(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || {
        use crate::schema::word_entries;

        let conn: &PgConnection = &pool.get().unwrap();

        let items = word_entries::table
            .order(word_entries::id).limit(30)
            .get_results::<WordEntry>(conn)?;

        return Ok(items)
    }).await;

    match res {
        Ok(queried_items) => Ok(HttpResponse::Ok().json(&queried_items)),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}