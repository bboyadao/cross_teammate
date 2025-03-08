// #![allow(non_snake_case, dead_code)]
// fn main() {
//     uniffi::generate_scaffolding("src/teammate.udl").unwrap();
// }

fn main() {
    if let Err(e) = uniffi::generate_scaffolding("src/teammate.udl") {
        eprintln!("Failed to generate UniFFI scaffolding: {:?}", e);
        std::process::exit(1);
    }
}