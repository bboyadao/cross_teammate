import { readFileSync } from "fs";
import { fileURLToPath } from "url";
import { dirname, join } from "path";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const wasmPath = join(__dirname, "wasm_bg.wasm");
const wasmBinary = readFileSync(wasmPath);

const { initSync, Ulid, User, Expense, Teammate } = await import("./wasm.js");

initSync({ module: wasmBinary });

function assert(condition, message) {
  if (!condition) {
    throw new Error(`Assertion failed: ${message}`);
  }
}

function assertEqual(actual, expected, message) {
  if (actual !== expected) {
    throw new Error(
      `${message || "Assertion failed"}: expected ${expected}, got ${actual}`
    );
  }
}

function testUlid() {
  console.log("Running testUlid...");
  const ulid = new Ulid();
  const value = ulid.toString();
  assert(value.length === 26, `ULID should be 26 chars, got ${value.length}`);

  const parsed = Ulid.fromString(value);
  assertEqual(parsed.toString(), value, "ULID round-trip");

  try {
    Ulid.fromString("invalid");
    assert(false, "Should have thrown on invalid ULID");
  } catch (e) {
    // expected
  }

  console.log("  ✓ testUlid");
}

function testUser() {
  console.log("Running testUser...");
  const user = new User("Alice");
  assertEqual(user.name, "Alice", "User name");
  assert(user.id.length === 26, "User id should be ULID length");

  console.log("  ✓ testUser");
}

function testExpense() {
  console.log("Running testExpense...");
  const user = new User("Alice");
  const expense = new Expense(user, 300n);
  assertEqual(expense.paid, 300n, "Expense paid");
  assertEqual(expense.amount, 300n, "Expense amount");

  console.log("  ✓ testExpense");
}

function testBasicCalculation() {
  console.log("Running testBasicCalculation...");
  const alice = new User("Alice");
  const bob = new User("Bob");

  const expenses = [new Expense(alice, 300n), new Expense(bob, 0n)];

  const tm = Teammate.from_expenses(expenses);
  const payments = tm.calculate();

  assertEqual(payments.length, 1, "Should have 1 payment");
  const p = payments[0];
  assertEqual(p.src, "Bob", "Src should be Bob");
  assertEqual(p.dst, "Alice", "Dst should be Alice");
  assertEqual(p.amount, 150n, "Amount should be 150");

  console.log("  ✓ testBasicCalculation");
}

function testEqualPayment() {
  console.log("Running testEqualPayment...");
  const alice = new User("Alice");
  const bob = new User("Bob");
  const charlie = new User("Charlie");

  const expenses = [
    new Expense(alice, 300n),
    new Expense(bob, 300n),
    new Expense(charlie, 300n),
  ];

  const tm = Teammate.from_expenses(expenses);
  const payments = tm.calculate();

  assertEqual(payments.length, 0, "No payments when everyone paid equally");

  console.log("  ✓ testEqualPayment");
}

function testReferenceData() {
  console.log("Running testReferenceData...");
  const raw = [
    ["Tuan Anh", 0],
    ["Truong", 500],
    ["Dat beo", 1300],
    ["Trung", 200],
    ["hai", 8000],
    ["Dung", 0],
    ["Tu", 5000],
    ["Baxom", 0],
  ];

  const expenses = raw.map(([name, paid]) => {
    return new Expense(new User(name), BigInt(paid));
  });

  const tm = Teammate.from_expenses(expenses);
  const payments = tm.calculate();

  const totalRedistributed = payments.reduce((sum, p) => sum + p.amount, 0n);
  assertEqual(totalRedistributed, 9250n, "Total redistributed should be 9250");

  const each = 15000n / BigInt(raw.length);
  for (const [name, paid] of raw) {
    if (paid < each) {
      const debt = each - BigInt(paid);
      const sent = payments
        .filter((p) => p.src === name)
        .reduce((sum, p) => sum + p.amount, 0n);
      assertEqual(sent, debt, `${name} should have sent ${debt}`);
    } else if (paid > each) {
      const credit = BigInt(paid) - each;
      const received = payments
        .filter((p) => p.dst === name)
        .reduce((sum, p) => sum + p.amount, 0n);
      assertEqual(received, credit, `${name} should have received ${credit}`);
    }
  }

  console.log("  ✓ testReferenceData");
}

function testSinglePayer() {
  console.log("Running testSinglePayer...");
  const alice = new User("Alice");
  const bob = new User("Bob");
  const charlie = new User("Charlie");

  const expenses = [
    new Expense(alice, 900n),
    new Expense(bob, 0n),
    new Expense(charlie, 0n),
  ];

  const tm = Teammate.from_expenses(expenses);
  const payments = tm.calculate();

  assertEqual(payments.length, 2, "Should have 2 payments");
  const totalPaid = payments.reduce((sum, p) => sum + p.amount, 0n);
  assertEqual(totalPaid, 600n, "Total paid should be 600");

  console.log("  ✓ testSinglePayer");
}

const isMainModule = import.meta.url === `file://${process.argv[1]}`;

if (isMainModule) {
  console.log("JavaScript SDK tests:\n");
  try {
    testUlid();
    testUser();
    testExpense();
    testBasicCalculation();
    testEqualPayment();
    testReferenceData();
    testSinglePayer();
    console.log("\nAll JavaScript SDK tests passed! ✅");
  } catch (e) {
    console.error(`\nTests failed: ${e.message}`);
    if (e.stack) {
      console.error(e.stack);
    }
    process.exit(1);
  }
}

export {
  testUlid,
  testUser,
  testExpense,
  testBasicCalculation,
  testEqualPayment,
  testReferenceData,
  testSinglePayer,
};
