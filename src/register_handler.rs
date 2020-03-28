use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;

use crate::database::DbPool;
use crate::errors::ServiceError;
use crate::models::{NewUser, SlimUser, User};
use crate::security::hash_password;

// UserData is used to extract data from a post request by the client
#[derive(Debug, Deserialize)]
pub struct UserData {
    pub username: String,
    pub password: String,
}

/// POST /register 
pub async fn register_user(
    user_data: web::Json<UserData>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServiceError> {
    let user_data = user_data.into_inner();
    let res = web::block(move || {
        query_create_user(
            user_data.username,
            user_data.password,
            pool,
        )
    })
    .await;

    match res {
        Ok(user) => Ok(HttpResponse::Ok().json(&user)),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}


/// Inserts new user row into the database
fn query_create_user(
    username: String,
    password: String,
    pool: web::Data<DbPool>,
) -> Result<SlimUser, crate::errors::ServiceError> {
    use crate::schema::users::dsl::users;
    let conn: &PgConnection = &pool.get().unwrap();

    // try hashing the password, else return the error that will be converted to ServiceError
    let password: String = hash_password(&password)?;
    dbg!(&password);

    let user = NewUser::from_details(username, password);
    let inserted_user: User =
        diesel::insert_into(users).values(&user).get_result(conn)?;
    dbg!(&inserted_user);
    return Ok(inserted_user.into());
}
