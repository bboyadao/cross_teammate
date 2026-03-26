use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct User {
    pub name: String,
    pub id: Ulid
}
