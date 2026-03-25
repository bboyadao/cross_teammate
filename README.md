# Cross Teammate Expense Splitter

A cross-platform library for splitting shared expenses among teammates. This project provides a core implementation in Rust and exposes it via WASM (for Web/JS) and UniFFI (for Python and other languages).

## Project Structure

The workspace consists of three main crates:

- **`mate_core`**: The core library containing the expense splitting logic, data models (User, Expense, Payment, Teammate), and algorithmic implementation.
- **`uni`**: UniFFI bindings that bridge the core logic to other languages like Python.
- **`wasm`**: WebAssembly bindings using `wasm-bindgen` for use in JavaScript environments.
- **`sdk/python`**: Generated Python SDK using UniFFI bindings.

## Features

- **Optimal Settlement**: Calculates the minimum number of transactions needed to settle all debts.
- **Multi-platform**: Use the same logic in Rust, Browser (WASM), and Python.
- **Type-safe Models**: Robust data structures for managing users and expenses.

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) (for WASM builds)
- Python 3.x (for Python SDK)

### Note on WASM build and `getrandom` v0.4
The `wasm` build uses `getrandom` v0.4 which requires the `wasm_js` backend for `wasm32-unknown-unknown` target. The `Makefile` includes the necessary `RUSTFLAGS` to enable this:
```bash
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' wasm-pack build --target web --out-dir ../sdk/js
```

### Building and Testing

The project includes a `Makefile` for common tasks:

```bash
# Run all tests (Core, Uni, WASM)
make test

# Build WASM package for JavaScript
make build-wasm

# Build Python SDK
make build-python

# Run SDK tests (after building)
make test-python
make test-js

# Clean build artifacts
make clean
```

## Usage Examples

### Rust (Core)

```rust
use mate_core::models::expenses::Expense;
use mate_core::models::teammate::Teammate;
use mate_core::models::users::User;

#[tokio::main]
async fn main() {
    let alice = User::new("Alice", None);
    let bob = User::new("Bob", None);
    
    // Alice paid 300, Bob paid 0
    let alice_expense = Expense {
        amount: 300,
        user: alice.clone(),
        paid: 300,
        have_to_pay: 0,
        need_to_earn: 0,
    };
    let bob_expense = Expense {
        amount: 0,
        user: bob.clone(),
        paid: 0,
        have_to_pay: 0,
        need_to_earn: 0,
    };

    let tm = Teammate::anew(vec![alice_expense, bob_expense]).await;
    let payments = tm.calculate();

    for p in payments {
        println!("{} pays {} to {}", p.get_src().get_name(), p.amount, p.dst.name);
        // Output: Bob pays 150 to Alice
    }
}
```

### JavaScript (WASM)

```javascript
import init, { JsTeammate, JsExpense, JsUser } from './pkg/wasm.js';

async function run() {
    await init();
    
    const alice = new JsUser("Alice");
    const bob = new JsUser("Bob");
    
    const expenses = [
        new JsExpense(alice, 300n),
        new JsExpense(bob, 0n),
    ];
    
    const tm = JsTeammate.from_expenses(expenses);
    const payments = tm.calculate();
    
    payments.forEach(p => {
        console.log(`${p.src} pays ${p.amount} to ${p.dst}`);
    });
}
```

### Python (SDK)

```python
from teammate import Teammate, Expense, Member, Ulid

# Initialize members
alice = Member(name="Alice", id=Ulid.new())
bob = Member(name="Bob", id=Ulid.new())

# Define expenses
# Note: have_to_pay, need_to_earn, and amount are typically managed by the library,
# but can be initialized for the calculation.
expenses = [
    Expense(user=alice, amount=300, paid=300, have_to_pay=0, need_to_earn=0),
    Expense(user=bob, amount=0, paid=0, have_to_pay=0, need_to_earn=0),
]

# Calculate settlements
# Teammate.anew expects a list of users and a list of expenses in UniFFI version
tm = Teammate.anew(users=[alice, bob], expenses=expenses)
payments = tm.calculate()

for p in payments:
    print(f"{p.get_src().get_name()} pays {p.amount} to {p.dst.name}")
```

## Running Tests

To verify the implementation, you can run:

```bash
# Run Rust core tests
cargo test -p mate_core

# Run all tests via Makefile
make test
```

The core logic is heavily tested in `mate_core/tests/teammate_test.rs`, covering various edge cases like uneven payments, empty groups, and single participants.
