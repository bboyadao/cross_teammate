use crate::models::user::User;

#[derive(Debug, Clone)]
pub struct Expense {
    pub amount: u64,
    pub user: User,
    pub paid: u64,
    pub have_to_pay: u64,
    pub need_to_earn: u64,
}

impl Expense {
    pub fn new(user: User, amount: u64, paid: u64, have_to_pay: u64, need_to_earn: u64) -> Self {
        Self {
            amount,
            user,
            paid,
            have_to_pay,
            need_to_earn,
        }
    }

    pub fn get_user(&self) -> User {
        self.user.clone()
    }

    pub fn get_amount(&self) -> u64 {
        self.amount
    }

    pub fn get_have_to_pay(&self) -> u64 {
        self.have_to_pay
    }

    pub fn get_need_to_earn(&self) -> u64 {
        self.need_to_earn
    }

    pub fn get_paid(&self) -> u64 {
        self.paid
    }
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
