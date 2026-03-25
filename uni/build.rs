#![allow(non_snake_case, dead_code)]
use camino::Utf8PathBuf;

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let udl_file: Utf8PathBuf = [manifest_dir.to_string(), "src".to_string(), "teammate.udl".to_string()]
        .iter()
        .collect();
    uniffi::generate_scaffolding(&udl_file).unwrap();
}