use surrealdb::sql::Thing;

#[derive(Debug)]
pub struct UserId {
	id: Thing,
}

impl UserId {
	#[inline]
	pub fn new(id: Thing) -> Self {
		Self { id }
	}
}

impl UserId {
	pub fn id(&self) -> &Thing {
		&self.id
	}
}
