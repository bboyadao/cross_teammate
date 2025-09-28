// #![allow(non_snake_case, dead_code)]

fn main() {
    // uniffi::build::generate_scaffolding("src/teammate.udl").unwrap();
    uniffi::generate_scaffolding("src/teammate.udl").unwrap()
}
