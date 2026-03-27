# Repository Guidelines

## Project Structure

This workspace contains three main Rust crates:

- **`src/`**: Entry point with modular organization:
  - `models/` — Data structures (e.g., `Member`)
  - `traits/` — Trait definitions
  - `impls/` — Trait implementations
- **`mate_core/`**: Core expense-splitting logic and models
- **`uni/`**: UniFFI bindings for Python and other languages
- **`wasm/`**: WebAssembly bindings for JavaScript

Generated SDKs are placed in:
- `sdk/python/` — Python SDK (generated via UniFFI)
- `sdk/js/` — JavaScript WASM package

## Build & Development Commands

Use the `Makefile` for common tasks:

| Command | Description |
|---------|-------------|
| `make test` | Run all tests (core, uni, WASM, Python, JS) |
| `make test-mate_core` | Test core crate only |
| `make build-wasm` | Build WASM for JavaScript |
| `make build-python` | Build Python SDK |
| `make test-python` | Run Python SDK tests |
| `make test-js` | Run JavaScript/WASM tests |
| `make clean` | Remove all build artifacts |

## Coding Style & Conventions

- **Rust**: Follow official [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- **Naming**:
  - Types: `PascalCase` (`Teammate`, `Expense`)
  - Functions/variables: `snake_case` (`calculate_settlements`)
  - Traits: `PascalCase` + verb (`TeammateCalculation`)
- **Formatting**: Run `cargo fmt` before committing
- **Linting**: Run `cargo clippy` to catch common issues

## Testing Guidelines

- **Frameworks**: `cargo test` (unit/integration), `pytest` (Python), `node` (JS)
- **Test files**:
  - `mate_core/tests/` — Core logic tests
  - `uni/tests/` — UniFFI binding tests
  - `sdk/python/test_sdk.py` — Python integration tests
- **Coverage**: Ensure all public APIs are tested; test edge cases (empty groups, uneven payments)

## Commit & PR Guidelines

- **Commit messages**: Use imperative mood, present tense (e.g., `Add Python SDK tests`, `Fix WASM getrandom config`)
- **PR requirements**:
  - Include a clear description of changes
  - Reference related issues (e.g., `Fixes #123`)
  - Ensure all tests pass (`make test`)
  - Update documentation if behavior changes
