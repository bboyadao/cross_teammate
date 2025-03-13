use uuid::Uuid;

pub trait Users {
    fn new(name: &str, id: Option<Uuid>) -> Self;
}