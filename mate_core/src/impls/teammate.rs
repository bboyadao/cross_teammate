use async_trait::async_trait;
use serde_json::Result as JsonResult;

use crate::models::expenses::Expense;
use crate::models::payments::Payment;
use crate::models::teammate::Teammate;
use crate::models::users::User;
use crate::traits::to_json::ToJson;

impl Teammate {
    // Uniffi constructor to match `teammate.udl`:
    // `constructor(sequence<User> users, sequence<Expense> expenses);`
    #[uniffi::constructor]
    pub fn anew(users: Vec<User>, expenses: Vec<Expense>) -> Self {
        Self { users, expenses }
    }

    /// Convenience constructor used by Rust callers/tests.
    /// Builds `users` from `expenses[i].user`.
    pub async fn anew_from_expenses(expenses: Vec<Expense>) -> Self {
        let users: Vec<User> = expenses.iter().map(|e| e.user.clone()).collect();
        Teammate { users, expenses }
    }

    /// Compute the minimal set of payments to settle all debts.
    ///
    /// Algorithm (mirrors the Python reference in `api/src/teammate/alg.py`):
    /// 1. Sum total paid and compute `each = total / n` (integer division).
    /// 2. Tag every expense with `have_to_pay` (debtor) or `need_to_earn` (creditor).
    /// 3. Sort creditors descending by `need_to_earn`, debtors ascending by `have_to_pay`.
    /// 4. Greedily match each creditor against debtors until their balance is zero.
    pub fn calculate(&self) -> Vec<Payment> {
        let n = self.expenses.len();
        if n == 0 {
            return vec![];
        }

        let total: u64 = self.expenses.iter().map(|e| e.paid).sum();
        let each = total / n as u64;

        // Build mutable balance records.
        struct Balance<'a> {
            user: &'a User,
            have_to_pay: u64,  // debtor: owes this much
            need_to_earn: u64, // creditor: is owed this much
        }

        let mut balances: Vec<Balance> = self
            .expenses
            .iter()
            .map(|e| {
                let (have_to_pay, need_to_earn) = if e.paid < each {
                    (each - e.paid, 0)
                } else {
                    (0, e.paid - each)
                };
                Balance { user: &e.user, have_to_pay, need_to_earn }
            })
            .collect();

        // Partition into creditors and debtors.
        let mut creditors: Vec<usize> = balances
            .iter()
            .enumerate()
            .filter(|(_, b)| b.need_to_earn > 0)
            .map(|(i, _)| i)
            .collect();
        let mut debtors: Vec<usize> = balances
            .iter()
            .enumerate()
            .filter(|(_, b)| b.have_to_pay > 0)
            .map(|(i, _)| i)
            .collect();

        // Sort: creditors desc by need_to_earn, debtors asc by have_to_pay.
        creditors.sort_by(|&a, &b| balances[b].need_to_earn.cmp(&balances[a].need_to_earn));
        debtors.sort_by(|&a, &b| balances[a].have_to_pay.cmp(&balances[b].have_to_pay));

        let mut payments = Vec::new();

        for &ci in &creditors {
            for &di in &debtors {
                if balances[ci].need_to_earn == 0 {
                    break;
                }
                let amount = balances[di].have_to_pay.min(balances[ci].need_to_earn);
                if amount == 0 {
                    continue;
                }
                payments.push(Payment {
                    src: balances[di].user.clone(),
                    dst: balances[ci].user.clone(),
                    amount,
                });
                balances[ci].need_to_earn -= amount;
                balances[di].have_to_pay -= amount;
            }
        }

        payments
    }
}

#[async_trait]
impl ToJson for Teammate {
    async fn to_json(&self) -> JsonResult<String> {
        serde_json::to_string(self)
    }
}
