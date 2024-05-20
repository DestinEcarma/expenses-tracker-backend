use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize)]
pub struct Record {
    id: Thing,
}

impl Record {
    pub fn id(&self) -> &Thing {
        &self.id
    }
}

#[derive(Debug, Deserialize)]
pub struct RecordOut {
    out: Vec<Thing>,
}

impl RecordOut {
    pub fn out(&self) -> &Vec<Thing> {
        &self.out
    }
}

#[derive(Debug)]
pub enum Error {
    AuthFailNoAuthTokenCookie,
    InvalidCategoryPayload,
    InvalidUserPayload,
    UsernameTaken,
    NotFound,
    Server(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::AuthFailNoAuthTokenCookie => {
                (StatusCode::UNAUTHORIZED, "AUTH_FAIL_NO_AUTH_TOKEN_COOKIE").into_response()
            }
            Error::InvalidCategoryPayload => {
                (StatusCode::BAD_REQUEST, "INVALID_CATEGORY_PAYLOAD").into_response()
            }
            Error::UsernameTaken => (StatusCode::CONFLICT, "USERNAME_TAKEN").into_response(),
            Error::InvalidUserPayload => {
                (StatusCode::BAD_REQUEST, "INVALID_USERNAME_OR_PASSWORD").into_response()
            }
            Error::NotFound => (StatusCode::NOT_FOUND, "NOT_FOUND").into_response(),
            Error::Server(msg) => {
                println!("UNHANDLED_SERVER_ERROR: {msg}");

                (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_SERVER_ERROR").into_response()
            }
        }
    }
}

pub struct CookieNames;

impl CookieNames {
    pub const AUTH_TOKEN: &'static str = "auth-token";
}

pub fn merge_json<T: Serialize, S: Serialize>(value1: T, value2: S) -> Value {
    let mut merged = json!(value1);

    if let Value::Object(map1) = &mut merged {
        if let Value::Object(map2) = json!(value2) {
            for (k, v) in map2 {
                map1.insert(k, v);
            }
        }
    }

    merged
}
