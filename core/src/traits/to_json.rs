use async_trait::async_trait;
use serde_json::Result as JsonResult;

#[async_trait]
pub trait ToJson {
    async fn to_json(&self) -> JsonResult<String>;
}
