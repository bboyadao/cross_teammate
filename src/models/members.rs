use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use ulid::Ulid;

pub fn new_ulid() -> ulid::Ulid {
    Ulid::new()
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct Member {
    pub name: String,
    #[serde(default = "new_ulid")]
    pub id: Ulid,
}
