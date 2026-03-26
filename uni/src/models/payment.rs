use crate::models::user::User;
use mate_core::models::payments::Payment as CorePayment;

#[derive(Debug, Clone)]
pub struct Payment {
    pub src: User,
    pub dst: User,
    pub amount: u64,
}

impl Payment {
    pub fn get_src(&self) -> User {
        self.src.clone()
    }

    pub fn get_dst(&self) -> User {
        self.dst.clone()
    }

    pub fn get_amount(&self) -> u64 {
        self.amount
    }
}


impl Payment {
    // No inherent `from()`; conversions are implemented via the `From<...>` trait.
}

impl From<CorePayment> for Payment {
    fn from(inner: CorePayment) -> Self {
        Self {
            src: inner.src.into(),
            dst: inner.dst.into(),
            amount: inner.amount,
        }
    }
}

impl From<Payment> for CorePayment {
    fn from(inner: Payment) -> Self {
        Self {
            src: inner.src.into(),
            dst: inner.dst.into(),
            amount: inner.amount,
        }
    }
}
