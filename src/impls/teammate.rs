use async_trait::async_trait;
use crate::models::teammate::{Teammate};
use crate::models::users::{User};
use crate::models::expenses::{Expense};
use crate::models::teammate::{Teammate as TeamMate};
use crate::traits::to_json::ToJson;
use serde_json::Result as JsonResult;
impl TeamMate {
    pub async fn anew(expenses: Vec<Expense>) -> Self{
        let users: Vec<User> = Vec::new();
        Teammate { users, expenses }
    }
    // pub async fn calculate(self) -> Result<Vec<Payment>, Box<dyn Error>> {
    //     todo!()
    // }

}
#[async_trait]
impl ToJson for Teammate {
    async fn to_json(&self) -> JsonResult<String> {
        serde_json::to_string(self)
    }
}
