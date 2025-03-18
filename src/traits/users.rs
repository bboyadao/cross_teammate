use crate::UlidFfi;

pub trait Users {
    fn new(name: &str, id: Option<UlidFfi>) -> Self;
}