use axum::{
    async_trait,
    extract::{FromRequestParts, Request},
    http::request::Parts,
    middleware::Next,
    response::Response,
    RequestPartsExt,
};

use surrealdb::sql::Thing;
use tower_cookies::Cookies;

use crate::api::{
    db::defs::{DBGlobalQuery, DBTables, ExtensionDB},
    defs::{CookieNames, Error, Record},
};

use super::ctx::RawUser;

pub async fn mw_require_auth(
    raw_user: Result<RawUser, Error>,
    req: Request,
    next: Next,
) -> Result<Response, Error> {
    raw_user?;

    Ok(next.run(req).await)
}

#[async_trait]
impl<T: Send + Sync> FromRequestParts<T> for RawUser {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &T) -> Result<Self, Self::Rejection> {
        let cookies = parts.extract::<Cookies>().await.unwrap();
        let db = parts.extract::<ExtensionDB>().await.unwrap();

        let auth_token = cookies
            .get(CookieNames::AUTH_TOKEN)
            .map(|c| c.value().to_string());

        let id = auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?;
        let user_id = Thing::from((DBTables::USER, id.as_str()));

        match get_user(&db, &user_id).await {
            Err(error) => Err(error),
            Ok(_) => Ok(RawUser::new(user_id)),
        }
    }
}

pub async fn get_user(db: &ExtensionDB, user_id: &Thing) -> Result<(), Error> {
    let record = db
        .query(DBGlobalQuery::SELECT_USER_ID)
        .bind(("user_id", user_id))
        .await
        .map_err(|e| Error::Server(e.to_string()))?
        .take::<Option<Record>>(0)
        .map_err(|e| Error::Server(e.to_string()))?;

    match record {
        None => Err(Error::AuthFailNoAuthTokenCookie),
        Some(_) => Ok(()),
    }
}
