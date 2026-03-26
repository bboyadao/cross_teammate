use crate::models::user::User;
use crate::models::expense::Expense;
use mate_core::models::teammate::Teammate as CoreTeammate;

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
            .map(crate::models::Payment::from)
            .collect()
    }
}

impl From<Teammate> for CoreTeammate {
    fn from(inner: Teammate) -> Self {
        Self {
            users: inner.users.into_iter().map(Into::into).collect(),
            expenses: inner.expenses.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<CoreTeammate> for Teammate {
    fn from(inner: CoreTeammate) -> Self {
        Self {
            users: inner.users.into_iter().map(Into::into).collect(),
            expenses: inner.expenses.into_iter().map(Into::into).collect(),
        }
    }
}
