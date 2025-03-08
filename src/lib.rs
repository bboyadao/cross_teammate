uniffi::setup_scaffolding!("teammate");

#[uniffi::export]
pub fn say_hello(name: String) -> String {
    format!("Hello, {} from Rust!", name)
}