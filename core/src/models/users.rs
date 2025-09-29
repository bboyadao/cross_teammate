use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use url::Url;
pub fn new_ulid() -> Ulid {
    Ulid::new()
}

pub fn new_url() -> Url {
    Url::parse("https://example.com/").unwrap()
}
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct User {
    pub name: String,
    #[serde(default = "new_ulid")]
    pub id: Ulid
}