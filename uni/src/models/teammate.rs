use crate::models::{Expense, User};

#[derive(Debug, Clone)]
pub struct Teammate {
    pub users: Vec<User>,
    pub expenses: Vec<Expense>,
}

impl Teammate {
    #[uniffi::constructor]
    pub fn anew(users: Vec<User>, expenses: Vec<Expense>) -> Self {
        Self { users, expenses }
    }

    pub fn calculate(&self) -> Vec<crate::models::Payment> {
        // Convert to core types, run the algorithm, convert back.
        let core_tm: mate_core::models::teammate::Teammate = self.clone().into();
        core_tm
            .calculate()
            .into_iter()
            .map(|p| crate::models::Payment::from(p))
            .collect()
    }
}

impl From<mate_core::models::teammate::Teammate> for Teammate {
    fn from(inner: mate_core::models::teammate::Teammate) -> Self {
        Self {
            users: inner.users.into_iter().map(From::from).collect(),
            expenses: inner.expenses.into_iter().map(From::from).collect(),
        }
    }
}

impl From<Teammate> for mate_core::models::teammate::Teammate {
    fn from(inner: Teammate) -> Self {
        mate_core::models::teammate::Teammate {
            users: inner.users.into_iter().map(Into::into).collect(),
            expenses: inner.expenses.into_iter().map(Into::into).collect(),
        }
    }
}
