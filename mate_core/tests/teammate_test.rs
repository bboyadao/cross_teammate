use mate_core::models::expenses::Expense;
use mate_core::models::teammate::Teammate;
use mate_core::traits::users::Users;
use mate_core::models::users::User;

// Helper: build an Expense quickly.
fn exp(name: &str, paid: u64) -> Expense {
    Expense {
        amount: paid,
        user: User::new(name, None),
        paid,
        have_to_pay: 0,
        need_to_earn: 0,
    }
}

// Helper: total amount of all payments.
fn total_paid(payments: &[mate_core::models::payments::Payment]) -> u64 {
    payments.iter().map(|p| p.amount).sum()
}

// ── basic construction ───────────────────────────────────────────────────────

#[tokio::test]
async fn test_teammate_creation() {
    let expenses = vec![exp("Alice", 50), exp("Bob", 100)];
    let tm = Teammate::anew(expenses).await;
    assert_eq!(tm.expenses.len(), 2);
    assert_eq!(tm.users.len(), 2);
    assert_eq!(tm.users[0].name, "Alice");
    assert_eq!(tm.users[1].name, "Bob");
}

// ── calculate: reference data from Python README ────────────────────────────

/// Mirrors the exact test data from `api/src/teammate/alg.py`:
/// total=15000, n=8, each=1875
///
/// Expected payments (from Python output):
///   Dat beo  → hai    575
///   Truong   → hai   1375
///   Trung    → hai   1675
///   Tuan Anh → hai   1875
///   Dung     → hai    625
///   Dung     → Tu    1250
///   Baxom    → Tu    1875
#[tokio::test]
async fn test_calculate_reference_data() {
    let raw = vec![
        ("Tuan Anh", 0u64),
        ("Truong", 500),
        ("Dat beo", 1300),
        ("Trung", 200),
        ("hai", 8000),
        ("Dung", 0),
        ("Tu", 5000),
        ("Baxom", 0),
    ];
    let expenses: Vec<Expense> = raw.iter().map(|(n, p)| exp(n, *p)).collect();
    let tm = Teammate::anew(expenses).await;

    let payments = tm.calculate();

    // Total redistributed == total owed by debtors == sum(max(0, each - paid))
    // each = 15000 / 8 = 1875
    // debtors: Tuan Anh 1875, Truong 1375, Dat beo 575, Trung 1675, Dung 1875, Baxom 1875 => 9250
    assert_eq!(total_paid(&payments), 9250);

    // Every payment must have a positive amount.
    assert!(payments.iter().all(|p| p.amount > 0));

    // Verify that no debtor ends up still owing (all have_to_pay settled).
    // We do this by summing per-debtor payments and comparing to expected debt.
    let each = 15000u64 / 8;
    for (name, paid) in &raw {
        if *paid < each {
            let debt = each - paid;
            let sent: u64 = payments
                .iter()
                .filter(|p| p.src.name == *name)
                .map(|p| p.amount)
                .sum();
            assert_eq!(sent, debt, "{name} should have sent {debt} but sent {sent}");
        }
    }

    // Verify every creditor received exactly what they are owed.
    for (name, paid) in &raw {
        if *paid > each {
            let owed = paid - each;
            let received: u64 = payments
                .iter()
                .filter(|p| p.dst.name == *name)
                .map(|p| p.amount)
                .sum();
            assert_eq!(received, owed, "{name} should receive {owed} but got {received}");
        }
    }
}

// ── edge cases ───────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_calculate_empty() {
    let tm = Teammate::anew(vec![]).await;
    assert!(tm.calculate().is_empty());
}

#[tokio::test]
async fn test_calculate_everyone_paid_equally() {
    let expenses = vec![exp("A", 100), exp("B", 100), exp("C", 100)];
    let tm = Teammate::anew(expenses).await;
    // No one owes anything.
    assert!(tm.calculate().is_empty());
}

#[tokio::test]
async fn test_calculate_one_person_paid_all() {
    // Alice paid 300, Bob and Charlie paid 0. each = 100.
    let expenses = vec![exp("Alice", 300), exp("Bob", 0), exp("Charlie", 0)];
    let tm = Teammate::anew(expenses).await;
    let payments = tm.calculate();

    // Both Bob and Charlie owe Alice 100 each.
    assert_eq!(total_paid(&payments), 200);
    for p in &payments {
        assert_eq!(p.dst.name, "Alice");
        assert_eq!(p.amount, 100);
    }
    let senders: Vec<&str> = payments.iter().map(|p| p.src.name.as_str()).collect();
    assert!(senders.contains(&"Bob"));
    assert!(senders.contains(&"Charlie"));
}

#[tokio::test]
async fn test_calculate_single_participant() {
    // Only one person — no payments needed regardless of amount paid.
    let tm = Teammate::anew(vec![exp("Solo", 500)]).await;
    assert!(tm.calculate().is_empty());
}

#[tokio::test]
async fn test_calculate_two_people_uneven() {
    // Alice paid 0, Bob paid 200. each = 100.
    let expenses = vec![exp("Alice", 0), exp("Bob", 200)];
    let tm = Teammate::anew(expenses).await;
    let payments = tm.calculate();

    assert_eq!(payments.len(), 1);
    assert_eq!(payments[0].src.name, "Alice");
    assert_eq!(payments[0].dst.name, "Bob");
    assert_eq!(payments[0].amount, 100);
}

#[tokio::test]
async fn test_calculate_all_zero_paid() {
    // Everyone paid 0 — each = 0, no payments.
    let expenses = vec![exp("A", 0), exp("B", 0), exp("C", 0)];
    let tm = Teammate::anew(expenses).await;
    assert!(tm.calculate().is_empty());
}

#[tokio::test]
async fn test_calculate_payments_are_non_negative() {
    let raw = vec![
        ("X", 1000u64),
        ("Y", 0),
        ("Z", 500),
        ("W", 300),
    ];
    let expenses: Vec<Expense> = raw.iter().map(|(n, p)| exp(n, *p)).collect();
    let tm = Teammate::anew(expenses).await;
    let payments = tm.calculate();
    assert!(payments.iter().all(|p| p.amount > 0));
}

// ── ToJson ───────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_to_json_roundtrip() {
    use mate_core::traits::to_json::ToJson;

    let expenses = vec![exp("Alice", 100), exp("Bob", 200)];
    let tm = Teammate::anew(expenses).await;
    let json = tm.to_json().await.expect("serialization failed");
    assert!(json.contains("Alice"));
    assert!(json.contains("Bob"));
}
