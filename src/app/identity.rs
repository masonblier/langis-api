use actix_identity::{CookieIdentityPolicy, IdentityService};

use crate::app::config;

/// Gets the identity service for injection into an actix-web app
pub fn get_identity_service() -> IdentityService<CookieIdentityPolicy> {
    IdentityService::new(
        CookieIdentityPolicy::new(config::SECRET_KEY.as_bytes())
            .name("auth")
            .path("/")
            .domain(config::APP_DOMAIN.as_str())
            .max_age_time(chrono::Duration::days(1))
            .secure(true)
    )
}
