use axum::Json;
use bcrypt::{hash, DEFAULT_COST};
use regex::Regex;
use serde_json::json;

use crate::api::{
    db::defs::{DBGlobalQuery, DBTables, ExtensionDB},
    defs::{Error, Record},
};

use super::defs::SignUpPayLoad;

static USERNAME_REGEX: &str = r"^[a-zA-Z0-9_]{3, 20}$";
static COULD_NOT_CREATE_USER: &str = "Could not create user.";

pub async fn username_exists(db: &ExtensionDB, username: &String) -> Result<(), Error> {
    let users = db
        .query(DBGlobalQuery::SELECT_USER_USERNAME)
        .bind(("username", username))
        .await
        .map_err(|e| Error::Server(e.to_string()))?
        .take::<Option<Record>>(0)
        .map_err(|e| Error::Server(e.to_string()))?;

    match users {
        Some(_) => Err(Error::UsernameTaken),
        None => Ok(()),
    }
}

pub async fn validate_payload(payload: &Json<SignUpPayLoad>) -> Result<(), Error> {
    let username_regex = Regex::new(USERNAME_REGEX).map_err(|e| Error::Server(e.to_string()))?;
    let valid_password = password_validate(&payload.password);

    match username_regex.is_match(&payload.username) && valid_password {
        false => Err(Error::InvalidUserPayload),
        true => Ok(()),
    }
}

pub fn password_validate(password: &String) -> bool {
    let mut has_digit = false;
    let mut has_lowercase = false;
    let mut has_uppercase = false;
    let mut has_special = false;

    for c in password.chars() {
        if c.is_digit(10) {
            has_digit = true;
        } else if c.is_lowercase() {
            has_lowercase = true;
        } else if c.is_uppercase() {
            has_uppercase = true;
        } else {
            has_special = true;
        }
    }

    has_digit && has_lowercase && has_uppercase && has_special
}

pub async fn hash_password(password: &String) -> Result<String, Error> {
    let hashed_password = hash(password, DEFAULT_COST).map_err(|e| Error::Server(e.to_string()))?;

    Ok(hashed_password)
}

pub async fn add_user(
    db: &ExtensionDB,
    username: &String,
    password: &String,
) -> Result<Record, Error> {
    let mut record = db
        .create::<Vec<Record>>(DBTables::USER)
        .content(json!({
            "username": username,
            "password": password,
        }))
        .await
        .map_err(|e| Error::Server(e.to_string()))?;

    if record.is_empty() {
        return Err(Error::Server(COULD_NOT_CREATE_USER.to_string()));
    }

    Ok(record.remove(0))
}
