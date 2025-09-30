pub use ulid::Ulid;

uniffi::custom_type!(Ulid, String, {
    remote,
    try_lift: |val| Ok(Ulid::from_string(&val)?),
    lower: |obj| obj.to_string(),
});
