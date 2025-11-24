use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use crate::models::expenses::Expense;
use crate::models::users::User;

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct Teammate {
    pub users: Vec<User>,
    pub expenses: Vec<Expense>,
}
