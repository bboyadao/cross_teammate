import init, { JsTeammate, JsExpense, JsUser } from './wasm.js';
import { readFileSync } from 'fs';
import { resolve, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));

async function runTests() {
    console.log("Initializing WASM...");
    const wasmBuffer = readFileSync(resolve(__dirname, 'wasm_bg.wasm'));
    await init(wasmBuffer);
    console.log("WASM initialized.");

    testBasicCalculation();
    testReferenceData();

    console.log("\nAll JS SDK tests passed!");
}

function testBasicCalculation() {
    console.log("Running testBasicCalculation...");
    const alice = new JsUser("Alice");
    const bob = new JsUser("Bob");

    const expenses = [
        new JsExpense(alice, 300n),
        new JsExpense(bob, 0n),
    ];

    const tm = JsTeammate.from_expenses(expenses);
    const payments = tm.calculate();

    if (payments.length !== 1) {
        throw new Error(`Expected 1 payment, got ${payments.length}`);
    }

    const p = payments[0];
    if (p.src !== "Bob" || p.dst !== "Alice" || p.amount !== 150n) {
        throw new Error(`Unexpected payment: ${p.src} pays ${p.amount} to ${p.dst}`);
    }
    console.log("testBasicCalculation passed!");
}

function testReferenceData() {
    console.log("Running testReferenceData...");
    const raw = [
        ["Tuan Anh", 0n],
        ["Truong", 500n],
        ["Dat beo", 1300n],
        ["Trung", 200n],
        ["hai", 8000n],
        ["Dung", 0n],
        ["Tu", 5000n],
        ["Baxom", 0n],
    ];

    const expenses = raw.map(([name, paid]) => {
        const user = new JsUser(name);
        return new JsExpense(user, paid);
    });

    const tm = JsTeammate.from_expenses(expenses);
    const payments = tm.calculate();

    const totalRedistributed = payments.reduce((sum, p) => sum + p.amount, 0n);
    if (totalRedistributed !== 9250n) {
        throw new Error(`Expected total redistributed 9250, got ${totalRedistributed}`);
    }

    const each = 15000n / 8n;
    raw.forEach(([name, paid]) => {
        if (paid < each) {
            const debt = each - paid;
            const sent = payments
                .filter(p => p.src === name)
                .reduce((sum, p) => sum + p.amount, 0n);
            if (sent !== debt) {
                throw new Error(`${name} should have sent ${debt} but sent ${sent}`);
            }
        } else if (paid > each) {
            const credit = paid - each;
            const received = payments
                .filter(p => p.dst === name)
                .reduce((sum, p) => sum + p.amount, 0n);
            if (received !== credit) {
                throw new Error(`${name} should have received ${credit} but received ${received}`);
            }
        }
    });

    console.log("testReferenceData passed!");
}

runTests().catch(err => {
    console.error("\nTests failed!");
    console.error(err);
    process.exit(1);
});
