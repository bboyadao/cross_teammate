use async_trait::async_trait;
use crate::traits::users::{ Users };
use crate::models::users::{new_uuid, User as ModelUser};
use crate::traits::to_json::ToJson;
use serde_json::Result as JsonResult;
use uuid::Uuid;

impl Users for ModelUser {
    fn new(name: &str, id: Option<Uuid>) -> Self {
        Self {
            name: name.to_string(),
            id: id.unwrap_or_else(new_uuid),
        }
    }
}

#[async_trait]
impl ToJson for ModelUser {
    async fn to_json(&self) -> JsonResult<String> {
        serde_json::to_string(self)
    }
}
