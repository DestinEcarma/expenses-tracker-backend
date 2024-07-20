use std::sync::Arc;

use axum::Extension;
use surrealdb::{engine::any::Any, Surreal};

pub type DB = Surreal<Any>;
pub type SharedDB = Arc<DB>;
pub type ExtensionDB = Extension<SharedDB>;

pub struct DBTables;

impl DBTables {
	pub const USER: &'static str = "user";
	pub const CATEGORY: &'static str = "category";
	pub const TRANSACTION: &'static str = "transaction";
}

pub struct DBGlobalQuery;

#[rustfmt::skip]
impl DBGlobalQuery {
	// Create new records
	pub const ADD_CATEGORY: &'static str = "fn::add_category($user_id, $name, $icon)";
	pub const ADD_TRANSACTION: &'static str = "fn::add_transation($category_id, $amount, $description)";

	// Select records
	pub const SELECT_USER: &'static str = "SELECT id FROM $user_id";
	pub const SELECT_CATEGORY: &'static str = "SELECT name, icon FROM $category_id";
	pub const SELECT_USER_BY_USERNAME: &'static str = "SELECT id, username, password FROM user WHERE username = $username";

	// Multiline queries
	pub const SELECT_USER_CATEGORIES: &'static str = r#"
	SELECT
		string::split(string::concat(id), ":")[1] AS id,
		name,
		icon,
		count(->category_transaction) AS transactions,
		math::sum((SELECT VALUE out.amount FROM ->category_transaction)) AS amount
	FROM $user_id->user_category.out;
	"#;

	pub const SELECT_CATEGORY_TRANSACTIONS: &'static str = r#"
	SELECT
		string::split(string::concat(id), ":")[1] AS id,
		amount,
		description,
		created_at
	FROM $category_id->category_transaction.out;
	"#;

	// Check ownership of records
	pub const CATEGORY_OWNERSHIP: &'static str = "fn::category_ownership($user_id, $category_id)";
	pub const TRANSACTION_OWNERSHIP: &'static str = "fn::transaction_ownership($category_id, $transaction_id)";

	// Update records
	pub const UPDATE_CATEGORY: &'static str = "UPDATE $category_id SET name = $name, icon = $icon";

	// Delete records
	pub const DELETE_CATEGORY: &'static str = "DELETE $category_id";
	pub const DELETE_TRANSACTION: &'static str = "DELETE $transaction_id";
}
