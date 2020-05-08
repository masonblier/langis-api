use std::io::{stdout, Write};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use crate::app::config;

/// Common type for database pool
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Creates an r2d2 postgresql database pool
pub fn get_database_pool() -> DbPool {
    // set up database connection pool
    writeln!(&mut stdout(), "Using database: {}", config::DATABASE_URI.as_str())
        .expect("Failed to print to stdout");
    let manager = ConnectionManager::<PgConnection>::new(config::DATABASE_URI.as_str());
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
