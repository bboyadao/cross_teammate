use async_trait::async_trait;
use crate::traits::users::{ Users };
use crate::models::users::{User as ModelUser};
use crate::traits::to_json::ToJson;
use serde_json::Result as JsonResult;

impl Users for ModelUser {
    fn new(name: &str, id: Option<ulid::Ulid>) -> Self {
        Self {
            name: name.to_string(),
            id: id.unwrap_or_else(ulid::Ulid::new),
        }
    }
}

#[async_trait]
impl ToJson for ModelUser {
    async fn to_json(&self) -> JsonResult<String> {
        serde_json::to_string(self)
    }
}
