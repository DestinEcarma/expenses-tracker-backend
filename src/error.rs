use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use core::fmt;
use derive_more::From;
use serde_json::json;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    // Client error
    AuthFailNoAuthTokenCookie,
    InvalidCategoryPayload,
    InvalidUserPayload,
    UsernameTaken,
    NotFound,

    // Server error
    SurrealdbSelectTransactionAmountIsNone,
    SurrealdbTransactionOwnershipIsNone,
    SurrealdbSelectUserCategoriesIsNone,
    SurrealdbCategoryTransactionsIsNone,
    SurrealdbSelectTransactionIsNone,
    SurrealdbCategoryOwnershipIsNone,
    SurrealdbSelectCategoryIsNone,
    SurrealdbCouldNotCreateQuery,
    IoError,
    #[from]
    ShuttleAxumError(shuttle_runtime::Error),
    #[from]
    SurrealdbError(surrealdb::Error),
    #[from]
    BcryptError(bcrypt::BcryptError),
    #[from]
    RegexError(regex::Error),
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match &self {
            Error::ShuttleAxumError(error) => println!("{self} :: {error}"),
            Error::SurrealdbError(error) => println!("{self} :: {error}"),
            Error::BcryptError(error) => println!("{self} :: {error}"),
            Error::RegexError(error) => println!("{self} :: {error}"),
            Error::SurrealdbSelectTransactionAmountIsNone
            | Error::SurrealdbTransactionOwnershipIsNone
            | Error::SurrealdbSelectUserCategoriesIsNone
            | Error::SurrealdbCategoryTransactionsIsNone
            | Error::SurrealdbSelectTransactionIsNone
            | Error::SurrealdbCategoryOwnershipIsNone
            | Error::SurrealdbSelectCategoryIsNone
            | Error::IoError
            | Error::SurrealdbCouldNotCreateQuery => println!("{self}"),
            _ => {
                return (
                    self.get_status_code(),
                    Json(json!({"message": self.to_string()})),
                )
                    .into_response()
            }
        }

        (
            self.get_status_code(),
            Json(json!({ "message": "SERVER_ERROR" })),
        )
            .into_response()
    }
}

impl fmt::Display for Error {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::AuthFailNoAuthTokenCookie => write!(f, "AUTH_FAIL_NO_AUTH_TOKEN_COOKIE"),
            Error::InvalidCategoryPayload => write!(f, "INVALID_CATEGORY_PAYLOAD"),
            Error::InvalidUserPayload => write!(f, "INVALID_USERNAME_OR_PASSWORD"),
            Error::UsernameTaken => write!(f, "USERNAME_TAKEN"),
            Error::NotFound => write!(f, "NOT_FOUND"),

            // Server error messages
            Error::SurrealdbSelectTransactionAmountIsNone => write!(f, "SURREALDB_SELECT_TRANSACTION_AMOUNT_IS_NONE"),
            Error::SurrealdbSelectUserCategoriesIsNone => write!(f, "SURREALDB_SELECT_USER_CATEGORIES_IS_NONE"),
            Error::SurrealdbCategoryTransactionsIsNone => write!(f, "SURREALDB_CATEGORY_TRANSACTIONS_IS_NONE"),
            Error::SurrealdbTransactionOwnershipIsNone => write!(f, "SURREALDB_TRANSACTION_OWNERSHIP_IS_NONE"),
            Error::SurrealdbSelectTransactionIsNone => write!(f, "SURREALDB_SELECT_TRANSACTION_IS_NONE"),
            Error::SurrealdbCategoryOwnershipIsNone => write!(f, "SURREALDB_CATEGORY_OWNERSHIP_IS_NONE"),
            Error::SurrealdbSelectCategoryIsNone => write!(f, "SURREALDB_SELECT_CATEGORY_IS_NONE"),
            Error::SurrealdbCouldNotCreateQuery => write!(f, "SURREALDB_COULD_NOT_CREATE_QUERY"),
            Error::IoError => write!(f, "IO_ERROR"),
            Error::ShuttleAxumError(_) => write!(f, "SHUTTLE_AXUM_ERROR"),
            Error::SurrealdbError(_) => write!(f, "SURREALDB_ERROR"),
            Error::BcryptError(_) => write!(f, "BCRYPT_ERROR"),
            Error::RegexError(_) => write!(f, "REGEX_ERROR"),
        }
    }
}

impl Error {
    fn get_status_code(&self) -> StatusCode {
        match self {
            Error::AuthFailNoAuthTokenCookie => StatusCode::UNAUTHORIZED,
            Error::InvalidCategoryPayload => StatusCode::BAD_REQUEST,
            Error::InvalidUserPayload => StatusCode::BAD_REQUEST,
            Error::UsernameTaken => StatusCode::CONFLICT,
            Error::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
