use bcrypt::verify;

use crate::api::{
    db::defs::{DBGlobalQuery, ExtensionDB},
    defs::Error,
};

use super::defs::UserLogin;

pub async fn get_user(db: &ExtensionDB, username: &String) -> Result<UserLogin, Error> {
    let result = db
        .query(DBGlobalQuery::SELECT_USER_USERNAME)
        .bind(("username", username))
        .await
        .map_err(|e| Error::Server(e.to_string()))?
        .take::<Option<UserLogin>>(0)
        .map_err(|e| Error::Server(e.to_string()))?;

    match result {
        None => Err(Error::InvalidUserPayload),
        Some(user) => Ok(user),
    }
}

pub async fn verify_password(password: &String, hashed_password: &String) -> Result<(), Error> {
    match verify(password, hashed_password).map_err(|e| Error::Server(e.to_string()))? {
        false => Err(Error::InvalidUserPayload),
        true => Ok(()),
    }
}
