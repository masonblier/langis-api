use std::pin::Pin;
use actix_identity::Identity;
use actix_web::{dev::Payload, web, Error, FromRequest, HttpRequest};
use chrono::{Utc};
use diesel::prelude::*;
use diesel::PgConnection;
use futures::future::Future;

use crate::app::database::DbPool;
use crate::app::errors::ServiceError;
use crate::app::models::{Session, SlimUser, User};

// extend SlimUser type for use as middleware
pub type SessionUser = SlimUser;

/// middleware for getting SlimUser data from cookie identity session
impl FromRequest for SessionUser {
    type Config = ();
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<SessionUser, Error>>>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        // get identity session_token from request
        let fut = Identity::from_request(req, pl);
        let pool_fut = web::Data::<DbPool>::from_request(req, pl);

        // get remote ip address from request
        let remote_ip = req.connection_info().remote().map(|s| s.to_string());

        Box::pin(async move {
            use crate::schema::sessions::dsl::{accessed_by_client_ip,
                last_accessed_at, token, sessions};
            use crate::schema::users::dsl::{id, users};

            if let Some(identity) = fut.await?.identity() {
                let pool = pool_fut.await?;
                let conn: &PgConnection = &pool.get().unwrap();

                // try finding user session
                let user_session = sessions
                  .filter(token.eq(&identity))
                  .get_result::<Session>(conn)
                  .map_err(|_| ServiceError::Unauthorized)?;

                // try finding user
                let user = users
                    .filter(id.eq(&user_session.user_id))
                    .get_result::<User>(conn)
                    .map_err(|_| ServiceError::Unauthorized)?;

                // update session last-access details
                diesel::update(sessions.find(&identity))
                  .set((
                      last_accessed_at.eq(Utc::now()),
                      accessed_by_client_ip.eq(remote_ip)
                  ))
                  .execute(conn)
                  .expect(&format!("Unable to update session {:?}", &identity));

                return Ok(SlimUser::from(user))
            } else {
                // Unauthorized if no identity found
                Err(ServiceError::Unauthorized.into())
            }
        })
    }
}
