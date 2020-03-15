use std::pin::Pin;

use actix_identity::Identity;
use actix_web::{
    dev::Payload, error::BlockingError, web, Error, FromRequest, HttpRequest,
    HttpResponse,
};
use diesel::prelude::*;
use diesel::PgConnection;
use futures::future::Future;
use serde::Deserialize;

use crate::errors::ServiceError;
use crate::models::{Session, SlimUser, User};
use crate::security::verify_password;
use crate::DbPool;

/// struct for storing login request data
#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub username: String,
    pub password: String,
}

// we need the same data
// simple aliasing makes the intentions clear and its more readable
pub type LoggedUser = SlimUser;

/// middleware for getting SlimUser data from cookie identity session
impl FromRequest for LoggedUser {
    type Config = ();
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<LoggedUser, Error>>>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        let fut = Identity::from_request(req, pl);
        let pool_fut = web::Data::<DbPool>::from_request(req, pl);

        Box::pin(async move {
            if let Some(identity) = fut.await?.identity() {
                let pool = pool_fut.await?;
                let conn: &PgConnection = &pool.get().unwrap();

                let user: LoggedUser = query_session_user(identity, conn)?;
                return Ok(user);
            };
            Err(ServiceError::Unauthorized.into())
        })
    }
}

/// DELETE /auth
pub async fn logout(
    id: Identity,
    pool: web::Data<DbPool>,
) -> HttpResponse {
    use crate::schema::sessions::dsl::{token, sessions};
    let conn: &PgConnection = &pool.get().unwrap();
    
    if let Some(session_token) = id.identity() {
        // try deleting the session from the db, dont care if it fails
        let _ = diesel::delete(
            sessions
            .filter(token.eq(&session_token))
        ).execute(conn);
    }

    id.forget();
    HttpResponse::Ok().finish()
}

/// POST /auth
pub async fn login(
    auth_data: web::Json<AuthData>,
    id: Identity,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServiceError> {
    let auth_data_inner: AuthData = auth_data.into_inner();

    let res = web::block(move || {
        let conn: &PgConnection = &pool.get().unwrap();
        let res = query_login(auth_data_inner, conn);

        match res {
            Ok(user) => {
                let new_session = Session::create(user.id);
                let created_session = query_create_session(new_session, conn).unwrap();
                Ok(created_session.token)
            }
            Err(err) => Err(err)
        }

    }).await;

    match res {
        Ok(token) => {
            id.remember(token);
            Ok(HttpResponse::Ok().finish())
        }
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

/// GET /auth
pub async fn get_me(logged_user: LoggedUser) -> HttpResponse {
    HttpResponse::Ok().json(logged_user)
}


/// Queries User object by given username and verifies if given password matches stored hash
fn query_login(auth_data: AuthData, conn: &PgConnection) -> Result<SlimUser, ServiceError> {
    use crate::schema::users::dsl::{username, users};
    let mut items = users
        .filter(username.eq(&auth_data.username))
        .load::<User>(conn)?;

    if let Some(user) = items.pop() {
        if let Ok(matching) = verify_password(&user.passhash, &auth_data.password) {
            if matching {
                return Ok(user.into());
            }
        }
    }
    Err(ServiceError::Unauthorized)
}

/// Inserts created Session object into the database
fn query_create_session(session: Session, conn: &PgConnection) -> Result<Session, ServiceError> {
    use crate::schema::sessions::dsl::{sessions};
    
    let inserted_session: Session =
        diesel::insert_into(sessions).values(&session).get_result(conn)?;
    dbg!(&inserted_session);
    return Ok(inserted_session.into());
}

/// Queries the database for session row by given token, and if successful, queries corresponding user data
fn query_session_user(session_token: String, conn: &PgConnection) -> Result<SlimUser, ServiceError> {
    use crate::schema::sessions::dsl::{token, sessions};
    use crate::schema::users::dsl::{id, users};
    
    let mut items = sessions
        .filter(token.eq(&session_token))
        .load::<Session>(conn)?;

    if let Some(session) = items.pop() {
        let mut items = users
            .filter(id.eq(&session.user_id))
            .load::<User>(conn)?;
        
        if let Some(user) = items.pop() {
            return Ok(SlimUser::from(user));
        }

    }
    Err(ServiceError::Unauthorized)
}