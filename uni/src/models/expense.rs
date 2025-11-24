use crate::models::user::User;

#[derive(Debug, Clone)]
pub struct Expense {
    pub amount: u64,
    pub user: User,
    pub paid: u64,
    pub have_to_pay: u64,
    pub need_to_earn: u64,
}

impl From<mate_core::models::expenses::Expense> for Expense {
    fn from(inner: mate_core::models::expenses::Expense) -> Self {
        Self {
            amount: inner.amount,
            user: inner.user.into(),
            paid: inner.paid,
            have_to_pay: inner.have_to_pay,
            need_to_earn: inner.need_to_earn,
        }
    }
}
impl From<Expense> for mate_core::models::expenses::Expense {
    fn from(inner: Expense) -> Self {
        mate_core::models::expenses::Expense {
            amount: inner.amount,
            user: inner.user.into(),
            paid: inner.paid,
            have_to_pay: inner.have_to_pay,
            need_to_earn: inner.need_to_earn,
        }
    }
}
