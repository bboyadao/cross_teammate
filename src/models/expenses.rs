use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use crate::models::users::User;

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct Expense {
    pub amount: u64,
    pub user: User,
    pub paid: u64,
    #[builder(default)]
    pub have_to_pay: u64,

    #[builder(default)]
    pub need_to_earn: u64,
}
