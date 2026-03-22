
pub trait MemberTrait {
    fn new(name: &str, id: Option<ulid::Ulid>) -> Self;
}
