use super::SignUpPayLoad;
use crate::{
    api::{
        db::defs::{DBGlobalQuery, DBTables, ExtensionDB},
        defs::Record,
    },
    error::{Error, Result},
};
use axum::Json;
use regex::Regex;
use serde_json::json;

static USERNAME_REGEX: &str = r"^[a-zA-Z0-9_]{3,20}$";

pub async fn username_exists(db: &ExtensionDB, username: &String) -> Result<()> {
    match db
        .query(DBGlobalQuery::SELECT_USER_BY_USERNAME)
        .bind(("username", username))
        .await?
        .take::<Option<Record>>(0)?
    {
        Some(_) => Err(Error::UsernameTaken),
        None => Ok(()),
    }
}

pub async fn validate_payload(payload: &Json<SignUpPayLoad>) -> Result<()> {
    let username_regex = Regex::new(USERNAME_REGEX)?;
    let valid_password = password_validate(&payload.password);

    match username_regex.is_match(&payload.username) && valid_password {
        false => Err(Error::InvalidUserPayload),
        true => Ok(()),
    }
}

pub fn password_validate(password: &str) -> bool {
    password.chars().any(char::is_numeric)
        && password.chars().any(char::is_lowercase)
        && password.chars().any(char::is_uppercase)
        && password.chars().any(|c| !c.is_alphanumeric())
}

pub async fn add_user(db: &ExtensionDB, username: &String, password: &String) -> Result<Record> {
    Ok(db
        .create::<Vec<Record>>(DBTables::USER)
        .content(json!({
            "username": username,
            "password": password,
        }))
        .await?
        .pop()
        .ok_or(Error::SurrealdbCouldNotCreateQuery)?)
}
