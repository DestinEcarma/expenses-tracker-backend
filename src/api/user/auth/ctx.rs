use surrealdb::sql::Thing;

pub struct RawUser {
    id: Thing,
}

impl RawUser {
    #[inline]
    pub fn new(id: Thing) -> Self {
        Self { id }
    }
}

impl RawUser {
    pub fn id(&self) -> &Thing {
        &self.id
    }
}
