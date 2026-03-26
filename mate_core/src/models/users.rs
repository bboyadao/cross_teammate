use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use ulid::Ulid;


pub fn new_ulid() -> Ulid {
    Ulid::new()
}

fn default_gen_ulid() -> Option<Ulid> {
    Some(new_ulid())
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct User {
    pub name: String,
    #[serde(default = "default_gen_ulid")]
    #[builder(default = "default_gen_ulid()")]
    pub id: Option<Ulid>
}
