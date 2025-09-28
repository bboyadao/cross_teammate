#[cfg(test)]
mod tests {
    use teammate::models::teammate::Teammate;
    use teammate::models::expenses::Expense;
    use teammate::models::users::{new_ulid, User};
    use teammate::traits::users::Users;
    use teammate::traits::to_json::{ToJson};

    #[tokio::test]
    async fn test_teammate_creation() {
        let user1 = User::new("Alice", None);
        let user2 = User::new("Bob", None);
        let expense1 = Expense {
            amount: 100,
            user: user1.clone(),
            paid: 50,
            have_to_pay: 0,
            need_to_earn: 0,
        };
        let expense2 = Expense {
            amount: 200,
            user: user2.clone(),
            paid: 100,
            have_to_pay: 0,
            need_to_earn: 0,
        };

        let teammate = Teammate::anew(vec![expense1, expense2]).await;

        // dbg!(&teammate.expenses);
        // dbg!(&teammate.to_json().await);
        // println!("{:?}", teammate.expenses);
        // assert_eq!(teammate.expenses.len(), 2);

        dbg!(&teammate.expenses);
        let json = match teammate.to_json().await {
            Ok(json) => json,
            Err(e) => panic!("to_json failed: {:?}", e),
        };
        dbg!(&json);
        println!("{:?}", teammate.expenses);
        assert_eq!(teammate.expenses.len(), 2);
    }
}