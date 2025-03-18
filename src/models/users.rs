use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use crate::UlidFfi;


pub fn new_ulid() -> UlidFfi {
    UlidFfi("".into())
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct User {
    pub name: String,
    #[serde(default = "new_ulid")]
    pub id: UlidFfi,
}