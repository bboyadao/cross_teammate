use crate::models::custom_ulid::{ Ulid};

pub fn new_ulid() -> Ulid { Ulid::new() }

#[derive(Debug, Clone)]
pub struct User {
    pub id: Ulid,
    pub name: String,
}

impl User {
    #[uniffi::constructor]
    pub fn new(name: String) -> Self {
        Self { id: new_ulid(), name }
    }
}

// Convert from/to core
impl From<mate_core::models::users::User> for User {
    fn from(inner: mate_core::models::users::User) -> Self {
        Self { id: inner.id, name: inner.name }
    }
}
impl From<User> for mate_core::models::users::User {
    fn from(inner: User) -> Self {
        mate_core::models::users::User { id: inner.id, name: inner.name }
    }
}
