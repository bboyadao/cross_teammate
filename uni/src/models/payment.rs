use crate::models::user::User;

#[derive(Debug, Clone)]
pub struct Payment {
    pub src: User,
    pub dst: User,
    pub amount: u64,
}

impl From<mate_core::models::payments::Payment> for Payment {
    fn from(inner: mate_core::models::payments::Payment) -> Self {
        Self { src: inner.src.into(), dst: inner.dst.into(), amount: inner.amount }
    }
}

impl From<Payment> for mate_core::models::payments::Payment {
    fn from(inner: Payment) -> Self {
        mate_core::models::payments::Payment { src: inner.src.into(), dst: inner.dst.into(), amount: inner.amount }
    }
}
