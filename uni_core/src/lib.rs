pub use mate_core::models::teammate::Teammate;
pub use mate_core::models::expenses::Expense;
pub use mate_core::models::users::User;
pub use mate_core::models::payments::Payment;
use ulid::Ulid;


uniffi::custom_type!(Ulid, String, {
    // Remote is required since `Ulid` is from a different crate
    remote,
    try_lift: |val| Ok(Ulid::from_string(&val)?),
    lower: |obj| obj.into(),
});


uniffi::include_scaffolding!("teammate");