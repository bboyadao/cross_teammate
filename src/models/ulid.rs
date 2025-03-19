use ulid::Ulid;
use url::Url;
uniffi::custom_type!(Url, String, {
    // Remote is required since `Url` is from a different crate
    remote,
    try_lift: |val| Ok(Url::parse(&val)?),
    lower: |obj| obj.into(),
});

uniffi::custom_type!(Ulid, String, {
    remote,
    try_lift: |val| Ok(Ulid::from_string(&val)?),
    lower: |obj| obj.into(),
});
