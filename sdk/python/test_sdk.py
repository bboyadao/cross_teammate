import sys
import os
import uuid

# Add the current directory to sys.path so we can import teammate
sys.path.append(os.path.dirname(os.path.abspath(__file__)))

from teammate import Teammate, Expense, User

def new_ulid():
    # ULID must be a 26-character string.
    # Let's generate a valid-looking one.
    import time
    import random
    
    # ULID alphabet (Crockford's Base32)
    ALPHABET = "0123456789ABCDEFGHJKMNPQRSTVWXYZ"
    
    # Time part (10 chars) - just using some fixed value for simplicity as it doesn't matter for tests
    # Random part (16 chars)
    res = "".join(random.choice(ALPHABET) for _ in range(26))
    return res

def test_basic_calculation():
    print("Running test_basic_calculation...")
    alice = User(name="Alice", id=new_ulid())
    bob = User(name="Bob", id=new_ulid())
    
    # Alice paid 300, Bob paid 0. Each should pay 150.
    # Bob owes Alice 150.
    expenses = [
        Expense(user=alice, amount=300, paid=300, have_to_pay=0, need_to_earn=0),
        Expense(user=bob, amount=0, paid=0, have_to_pay=0, need_to_earn=0),
    ]
    
    tm = Teammate.anew(users=[alice, bob], expenses=expenses)
    payments = tm.calculate()
    
    assert len(payments) == 1
    p = payments[0]
    assert p.src.name == "Bob"
    assert p.dst.name == "Alice"
    assert p.amount == 150
    print("test_basic_calculation passed!")

def test_reference_data():
    print("Running test_reference_data...")
    # Reference data from mate_core/tests/teammate_test.rs
    raw = [
        ("Tuan Anh", 0),
        ("Truong", 500),
        ("Dat beo", 1300),
        ("Trung", 200),
        ("hai", 8000),
        ("Dung", 0),
        ("Tu", 5000),
        ("Baxom", 0),
    ]
    
    users = []
    expenses = []
    for name, paid in raw:
        u = User(name=name, id=new_ulid())
        users.append(u)
        expenses.append(Expense(user=u, amount=paid, paid=paid, have_to_pay=0, need_to_earn=0))
        
    tm = Teammate.anew(users=users, expenses=expenses)
    payments = tm.calculate()
    
    # Total redistributed should be 9250
    total_redistributed = sum(p.amount for p in payments)
    assert total_redistributed == 9250
    
    # Check individual debts/credits
    each = 15000 // 8
    for name, paid in raw:
        if paid < each:
            debt = each - paid
            sent = sum(p.amount for p in payments if p.src.name == name)
            assert sent == debt, f"{name} should have sent {debt} but sent {sent}"
        elif paid > each:
            credit = paid - each
            received = sum(p.amount for p in payments if p.dst.name == name)
            assert received == credit, f"{name} should have received {credit} but received {received}"

    print("test_reference_data passed!")

if __name__ == "__main__":
    try:
        test_basic_calculation()
        test_reference_data()
        print("\nAll Python SDK tests passed!")
    except Exception as e:
        print(f"\nTests failed: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)
