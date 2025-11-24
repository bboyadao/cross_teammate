use uniffi::custom_type;
use ulid::Ulid as RawUlid; // Alias to avoid collision
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};

// Local wrapper named Ulid to match UDL
#[derive(Debug)]
pub struct Ulid(RawUlid);

// Optional: Forward methods for ergonomics
impl Ulid {
    pub fn new() -> Self {
        Ulid(RawUlid::new())
    }

    pub fn datetime(&self) -> std::time::SystemTime {
        self.0.datetime()
    }
}

// Define custom serialization for Ulid
custom_type!(Ulid, String, {
    try_lift: |s: String| {
        RawUlid::from_string(&s)
            .map(Ulid)
            .map_err(|_| anyhow!("Invalid ULID"))
    },
    lower: |ulid| ulid.0.to_string(),
});

// Error enum matching UDL
#[derive(thiserror::Error, uniffi::Error, Debug)]
pub enum Error {
    #[error("Invalid ULID")]
    InvalidUlid,
    #[error("Internal error: {0}")]
    InternalError(String),
};

// UDL functions
#[uniffi::export]
pub fn generate_ulid() -> Result<Ulid, Error> {
    Ok(Ulid::new())
}

#[uniffi::export]
pub fn get_ulid_timestamp(ulid: Ulid) -> Result<String, Error> {
    let system_time = ulid.datetime();
    let datetime: DateTime<Utc> = system_time
        .try_into()
        .map_err(|e| Error::InternalError(format!("Failed to convert timestamp: {}", e)))?;
    Ok(datetime.to_rfc3339())
}

// Include scaffolding
// uniffi::include_scaffolding!("teammate");/
// Include scaffolding
// uniffi::include_scaffolding!("teammate");

// Include scaffolding
// uniffi::include_scaffolding!("teammate");



// Include scaffolding
// uniffi::include_scaffolding!("teammate");
// And implement the rest of your interface...

// use url::Url;
//
// uniffi::custom_type!(Url, String, {
//     // Remote is required since `Url` is from a different crate
//     remote,
//     try_lift: |val| Ok(Url::parse(&val)?),
//     lower: |obj| obj.into(),
// });

// uniffi::custom_type!(Ulid, String, {
//     remote,
//     lower: |s| s.to_string(),
//     try_lift: |s| Ok(ulid::Ulid::from_string(&s)?),
// });
//
// uniffi::custom_type!(Ulid, String, {
//     remote,
//     try_lift: |val| Ok(Ulid::from_string(&val)?),
//     lower: |obj| obj.into(),
// });

// uniffi::use_remote_type!(ulid::Ulid);
// type ULID = ulid::Ulid;
// type AE = anyhow::Error;
// #[uniffi::remote(Object)]
// struct ULID;

// #[uniffi::remote(Object)]
// struct AE;
