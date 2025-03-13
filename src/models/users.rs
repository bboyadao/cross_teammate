use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn new_uuid() -> Uuid {
    Uuid::new_v4()
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct User {
    pub name: String,
    #[serde(default = "new_uuid")]
    pub id: Uuid,
}