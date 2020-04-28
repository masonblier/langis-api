lazy_static::lazy_static! {
    // server bind address, default to `127.0.0.1:8301`
    pub static ref BIND_ADDRESS: String = std::env::var("BIND_ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1:8301".to_string());

    // app domain, default to `localhost`
    pub static ref APP_DOMAIN: String = std::env::var("DOMAIN")
        .unwrap_or_else(|_| "localhost".to_string());

    // postgres database uri
    pub static ref DATABASE_URI: String = std::env::var("DATABASE_URL")
        .expect("\n\n  DATABASE_URI environment variable required for postgres connection\n\n");
}
