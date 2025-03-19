pub mod traits;
pub mod models;
pub mod impls;

use ulid::Ulid;
use crate::models::teammate::Teammate;
use crate::models::users::User;
use crate::models::expenses::Expense;
use crate::models::payments::Payment;
uniffi::include_scaffolding!("teammate");