use uuid::Uuid;
pub mod traits;
pub mod models;
pub mod impls;

pub struct CusUuid(pub Uuid);
use crate::models::teammate::Teammate;
use crate::models::users::User;
use crate::models::expenses::Expense;
use crate::models::payments::Payment;

uniffi::include_scaffolding!("teammate");