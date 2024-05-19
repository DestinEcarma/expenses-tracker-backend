use std::sync::Arc;

use axum::Extension;
use surrealdb::{engine::any::Any, Surreal};

pub type DB = Surreal<Any>;
pub type SharedDB = Arc<DB>;
pub type ExtensionDB = Extension<SharedDB>;

pub struct DBTables;

impl DBTables {
    pub const USER: &'static str = "user";
    // pub const CATEGORY: &'static str = "category";
    // pub const TRANSACTION: &'static str = "transaction";
    // pub const USER_CATEGORIES: &'static str = "user_categories";
    // pub const USER_TRANSACTIONS: &'static str = "user_transactions";
}

pub struct DBGlobalQuery;

#[rustfmt::skip]
impl DBGlobalQuery {
    pub const ADD_CATEGORY: &'static str = "fn::add_category($user_id, $name, $icon)";
    pub const ADD_TRANSACTION: &'static str = "fn::add_transation($category_id, $amount, $description)";
    pub const SELECT_USER_ID: &'static str = "SELECT id FROM $user_id";
    pub const SELECT_CATEGORY: &'static str = "SELECT name, icon FROM $category_id";
    pub const SELECT_TRANSACTION_GET: &'static str = "SELECT * FROM $transaction_id";
    pub const SELECT_TRANSACTION_AMOUNT: &'static str = "SELECT amount FROM $transaction_id";
    pub const SELECT_USER_USERNAME: &'static str = "SELECT id, username, password FROM user WHERE username=$username";
    pub const SELECT_USER_GET_CATEGORIES: &'static str = "SELECT ->user_category.out as out FROM $user_id";
    pub const SELECT_CATEGORY_GET_TRANSACTIONS: &'static str = "SELECT ->category_transaction.out as out FROM $category_id";
    pub const CATEGORY_OWNERSHIP: &'static str = "fn::category_ownership($user_id, $category_id)";
    pub const TRANSACTION_OWNERSHIP: &'static str = "fn::transaction_ownership($category_id, $transaction_id)";
    pub const DELETE_TRANSACTION: &'static str = "DELETE $transaction_id";
    pub const DELETE_CATEGORY: &'static str = "DELETE $category_id";
}
