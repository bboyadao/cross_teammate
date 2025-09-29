pub mod traits;
pub mod models;
pub mod impls;
pub use ulid::Ulid;
pub fn new_ulid() -> Ulid {
    Ulid::new()
}

pub fn ulid_to_string(id: &Ulid) -> String {
    id.to_string()
}

pub fn ulid_from_string(s: &str) -> Result<Ulid, String> {
    Ulid::from_string(s).map_err(|e| e.to_string())
}