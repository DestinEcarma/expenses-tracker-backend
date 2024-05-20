use axum::Json;
use regex::Regex;

use crate::api::defs::Error;

use super::defs::CategoryPayload;

static CATEGORY_NAME_REGEX: &str = r"^.{3,}$";

pub async fn validate_payload(payload: &Json<CategoryPayload>) -> Result<(), Error> {
    let name_regex = Regex::new(CATEGORY_NAME_REGEX).map_err(|e| Error::Server(e.to_string()))?;

    match name_regex.is_match(&payload.name) {
        false => Err(Error::InvalidCategoryPayload),
        true => Ok(()),
    }
}
