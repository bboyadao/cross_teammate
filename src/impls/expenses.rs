use serde_json::Result as JsonResult;
use async_trait::async_trait;
use crate::models::expenses::Expense;
use crate::traits::to_json::ToJson;

#[async_trait]
impl ToJson for Expense {
    async fn to_json(&self) -> JsonResult<String> {
        serde_json::to_string(self)
    }
}