use serde::{Deserialize, Serialize};

use crate::models::users::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub src: User,
    pub dst: User,
    pub amount: u64,
}
