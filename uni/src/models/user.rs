use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use crate::models::custom_ulid::Ulid;
use mate_core::models::users::User as CoreUser;

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct User {
    pub name: String,
    pub id: Ulid
}

impl User {
    pub fn new(name: String, id: Option<Ulid>) -> Self {
        Self {
            name,
            id: id.unwrap_or_else(ulid::Ulid::new),
        }
    }
}

// Convert to/from core models so the FFI layer can delegate to `mate_core` without duplicating logic.
impl From<CoreUser> for User {
    fn from(inner: CoreUser) -> Self {
        Self {
            id: inner.id,
            name: inner.name,
        }
    }
}

impl From<User> for CoreUser {
    fn from(inner: User) -> Self {
        CoreUser {
            id: inner.id,
            name: inner.name,
        }
    }
}

