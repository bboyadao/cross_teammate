use async_trait::async_trait;
use crate::traits::members::MemberTrait;
use crate::models::members::{new_ulid, Member};
use crate::traits::to_json::ToJson;
use serde_json::Result as JsonResult;

impl MemberTrait for Member {
    fn new(name: &str, id: Option<ulid::Ulid>) -> Self {
        Self {
            name: name.to_string(),
            id: id.unwrap_or_else(new_ulid),
        }
    }
}

#[async_trait]
impl ToJson for Member {
    async fn to_json(&self) -> JsonResult<String> {
        serde_json::to_string(self)
    }
}
