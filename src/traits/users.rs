
pub trait Users {
    fn new(name: &str, id: Option<ulid::Ulid>) -> Self;
}