pub mod traits;
pub mod models;
pub mod impls;
use crate::models::teammate::Teammate;
use crate::models::users::User;
use crate::models::expenses::Expense;
use crate::models::payments::Payment;

use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ulid(pub u128);

// Custom conversions between Ulid and String for UniFFI
impl From<Ulid> for String {
    fn from(val: Ulid) -> Self {
        ulid::Ulid::from(val.0).to_string()
    }
}

impl TryFrom<String> for Ulid {
    type Error = ulid::DecodeError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let ulid = ulid::Ulid::from_str(&s)?;
        Ok(Ulid(ulid.into()))
    }
}

// Helper struct for UniFFI to handle Ulid as String
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UlidFfi(String);

// Conversion between Ulid and UlidFfi
impl From<Ulid> for UlidFfi {
    fn from(ulid: Ulid) -> Self {
        UlidFfi(ulid.into())
    }
}

impl TryFrom<UlidFfi> for Ulid {
    type Error = ulid::DecodeError;

    fn try_from(ffi: UlidFfi) -> Result<Self, Self::Error> {
        Ulid::try_from(ffi.0)
    }
}

// Register the FFI-compatible struct with UniFFI
uniffi::custom_newtype!(UlidFfi, String);

uniffi::include_scaffolding!("teammate");