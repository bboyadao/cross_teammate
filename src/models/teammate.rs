use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use crate::models::expenses::Expense;
use crate::models::users::User;
use uuid::Uuid;

uniffi::custom_type!(Uuid, String, {
    remote,
    try_lift: |val| Ok(Uuid::parse_str(&val)?),
    lower: |obj| obj.into(),
});


#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct Teammate {
    pub users: Vec<User>,
    pub expenses: Vec<Expense>,
}
