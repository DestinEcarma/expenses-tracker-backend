use axum::Json;
use regex::Regex;

use super::defs::CategoryPayload;
use crate::error::{Error, Result};

static CATEGORY_NAME_REGEX: &str = r"^.{3,}$";

pub async fn validate_payload(payload: &Json<CategoryPayload>) -> Result<()> {
    let name_regex = Regex::new(CATEGORY_NAME_REGEX)?;

    match name_regex.is_match(&payload.name) {
        false => Err(Error::InvalidCategoryPayload),
        true => Ok(()),
    }
}
