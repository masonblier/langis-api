use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::prelude::*;
use itertools::multizip;
use serde::{Deserialize,Serialize};

use crate::app::database::DbPool;
use crate::app::errors::ServiceError;
use crate::app::models::{WordEntry,WordEntryNote,WordEntryReading,WordEntryTag};

const PER_PAGE: i64 = 30;

/// GET /word_entries list params
#[derive(Deserialize)]
pub struct ListWordEntriesParams {
    pub query: String,
    pub page: i64,
}
/// GET /word_entries list result
#[derive(Debug, Deserialize,Serialize)]
pub struct ListWordEntriesResult {
    pub page: Vec<ListWordEntriesResultRecord>,
    pub page_count: i64,
}
#[derive(Debug, Deserialize,Serialize)]
pub struct ListWordEntriesResultRecord {
    pub word_entry: WordEntry,
    pub word_entry_notes: Vec<WordEntryNote>,
    pub word_entry_readings: Vec<WordEntryReading>,
    pub word_entry_tags: Vec<WordEntryTag>,
}
/// GET /word_entries list endpoint
pub async fn list_word_entries(
    params: web::Query<ListWordEntriesParams>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || {
        use diesel::dsl::count_star;
        use crate::schema::word_entries;
        use crate::schema::word_entries::dsl::{orth,quote};

        let conn: &PgConnection = &pool.get().unwrap();

        // query word entries page
        let offset = (params.page - 1) * PER_PAGE;
        let query = &params.query;
        let word_entries_items = word_entries::table
            .filter(orth.ilike(query.clone()).or(quote.ilike(query.clone())))
            .order(word_entries::id).offset(offset).limit(PER_PAGE)
            .get_results::<WordEntry>(conn)?;
        let count: i64 = word_entries::table
            .select(count_star())
            .filter(orth.ilike(query.clone()).or(quote.ilike(query.clone())))
            .first(conn)?;

        // get joined records
        let word_entry_notes_items = WordEntryNote::belonging_to(&word_entries_items)
            .get_results::<WordEntryNote>(conn)?
            .grouped_by(&word_entries_items);
        let word_entry_readings_items = WordEntryReading::belonging_to(&word_entries_items)
            .get_results::<WordEntryReading>(conn)?
            .grouped_by(&word_entries_items);
        let word_entry_tags_items = WordEntryTag::belonging_to(&word_entries_items)
            .get_results::<WordEntryTag>(conn)?
            .grouped_by(&word_entries_items);

        // zip items
        let entries = multizip((word_entries_items, word_entry_notes_items,
            word_entry_readings_items, word_entry_tags_items)).map({|t|
                ListWordEntriesResultRecord {
                    word_entry: t.0,
                    word_entry_notes: t.1,
                    word_entry_readings: t.2,
                    word_entry_tags: t.3,
                }
            }).collect::<Vec<_>>();

        // result
        return Ok(ListWordEntriesResult {
            page: entries,
            page_count: (count as f64 / PER_PAGE as f64).ceil() as i64,
        })
    }).await;

    match res {
        Ok(result) => Ok(HttpResponse::Ok().json(&result)),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}