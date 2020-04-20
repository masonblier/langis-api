#[cfg(test)]
pub mod tests {
    use std::io::{stdout, Write};
    use actix_http::{Request,cookie::Cookie};
    use actix_service::Service;
    use actix_web::dev::ServiceResponse;
    use actix_web::{http, test, web, App, body::Body, Error};
    use diesel::prelude::*;
    use diesel::sql_types::Text;
    use diesel_migrations::run_pending_migrations;

    use crate::app::controllers::auth_controller::AuthRequestData;
    use crate::app::database::{get_database_pool, DbPool};
    use crate::app::identity::get_identity_service;
    use crate::app::security::hash_password;
    use crate::app::routes::build_routes;

    // alias for test app type
    pub trait TestApp = Service<Request = Request, Response = ServiceResponse<Body>, Error = Error>;

    /// Testing setup function to reset testing database before any tests are run
    struct TestDbSetup {
        pool: DbPool
    }
    impl TestDbSetup {
        fn new() -> Self {
            // ensure test env config
            dotenv::from_filename(".env.test").ok();

            // create database pool
            let pool = get_database_pool();
            // connection for reset commands
            let conn: &PgConnection = &pool.get().unwrap();

            // drop all tables from database
            writeln!(&mut stdout(), "\nResetting database, dropping all tables...")
                .expect("Failed to print to stdout");
            #[derive(QueryableByName)]
            struct PgTablesEntry(
                #[column_name = "tablename"]
                #[sql_type = "Text"]
                String
            );
            let table_entries = diesel::sql_query(
                "SELECT tablename FROM pg_tables WHERE schemaname = 'public'"
            ).get_results::<PgTablesEntry>(conn).expect("Error when selecting table list");
            table_entries.into_iter().for_each(|entry| {
                diesel::sql_query(format!("DROP TABLE {} CASCADE", entry.0))
                    .execute(conn).expect("Error when attempting to drop table");
            });

            // run migrations
            run_pending_migrations(conn).expect("Error during database migration");
            // print separating newline
            writeln!(&mut stdout(), "").expect("Failed to print to stdout");

            TestDbSetup { pool }
        }
    }
    lazy_static::lazy_static! {
        static ref TEST_DB_POOL : TestDbSetup = TestDbSetup::new();
    }


    /// Creates App service with test configuration
    pub async fn create_test_app() -> impl TestApp {
        test::init_service(
            App::new()
                // set up DB pool to be used with web::Data<Pool> extractor
                .data(TEST_DB_POOL.pool.clone())
                // identity
                .wrap(get_identity_service())
                // json request parsing config
                .data(web::JsonConfig::default().limit(4096))
                .configure(build_routes)
        )
        .await
    }
}