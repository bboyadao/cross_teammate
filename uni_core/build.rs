#![allow(non_snake_case, dead_code)]

// fn main() {
//     uniffi::generate_scaffolding("teammate.udl").unwrap()
// }

use camino::Utf8PathBuf;

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let udl_file = Utf8PathBuf::from(manifest_dir).join("teammate.udl");
    uniffi::generate_scaffolding(&udl_file).unwrap();
}