use crate::models::custom_ulid::Ulid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Ulid,
    pub name: String,
}

impl User {
    /// Create a user. If `id` is empty string, a fresh ULID is generated automatically.
    pub fn new(name: String, id: String) -> Self {
        let id_value = if id.is_empty() {
            Ulid::new()
        } else {
            Ulid::from_string(&id).expect("Invalid ULID string")
        };
        
        Self {
            id: id_value,
            name,
        }
    }

    pub fn from_name(name: String) -> Self {
        Self::new(name, String::new())
    }

    pub fn get_id(&self) -> String {
        self.id.to_string()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

// Convert from/to core
impl From<mate_core::models::users::User> for User {
    fn from(inner: mate_core::models::users::User) -> Self {
        Self {
            id: inner.id,
            name: inner.name,
        }
    }
}

impl From<User> for mate_core::models::users::User {
    fn from(inner: User) -> Self {
        mate_core::models::users::User {
            id: inner.id,
            name: inner.name,
        }
    }
}
