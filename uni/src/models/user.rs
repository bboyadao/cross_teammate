use crate::models::custom_ulid::Ulid;
use mate_core::models::users::{new_ulid};
use mate_core::models::users::User as CoreUser;
#[derive(Debug, Clone)]
pub struct User {
    pub id: Option<Ulid>,
    pub name: String,
}

impl User {
    /// Create a user. If `id` is empty string, a fresh ULID is generated automatically.
    pub fn new(name: String, id: String) -> User {
        let id_value = if id.is_empty() {
            Some(new_ulid())
        } else {
            Some(Ulid::from_string(&id).expect("Invalid ULID string"))
        };
        
        User {
            id: id_value,
            name,
        }
    }

    pub fn get_id(&self) -> String {
        self.id.as_ref().map(|v| v.to_string()).unwrap_or_default()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
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
