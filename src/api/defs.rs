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
