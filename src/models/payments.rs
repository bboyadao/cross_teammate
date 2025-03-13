use crate::models::users::User;
pub struct Payment {
    pub src: User,
    pub dst: User,
    pub amount: u64,
}
