use std::env;
use diesel::{Connection, PgConnection, r2d2::ConnectionManager};
use dotenv::dotenv;

/// Common type for database pool
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// non-pooled connection to postgres database
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}