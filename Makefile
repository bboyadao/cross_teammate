# Makefile
CARGO = cargo
WASM_PACK = wasm-pack
TEST_FLAGS = --verbose --color=always
CRATES = mate_core uni wasm

.PHONY: test $(CRATES) test-all clean help build-wasm build-python test-python test-js

# Main test target
test: $(addprefix test-,$(CRATES)) test-python test-js

# Dynamic targets for each crate
test-mate_core:
	@echo "Testing core..."
	@cd mate_core && $(CARGO) test $(TEST_FLAGS)

test-uni:
	@echo "Testing uni (FFI bindings)..."
	@cd uni && $(CARGO) test $(TEST_FLAGS)

test-wasm:
	@echo "Testing wasm..."
	@cd wasm && $(CARGO) test $(TEST_FLAGS)

# Test specific crate
%: test-%
	@# This allows 'make core' as alias for 'make test-core'

# Test everything
test-all: test

# Build WASM for JS
build-wasm:
	@echo "Building WASM for JS..."
	@cd wasm && RUSTFLAGS='--cfg getrandom_backend="wasm_js"' $(WASM_PACK) build --target web --out-dir ../sdk/js

# Build Python SDK (renamed from buin)
build-python:
	@echo "Building Python SDK..."
	@mkdir -p sdk/python
	@cd uni && $(CARGO) build --release
	@cd uni && $(CARGO) run -p uni_core --bin uniffi-bindgen generate \
	  --library ../target/release/libuni_core.so \
	  --crate uni_core \
	  --language python \
	  --out-dir ../sdk/python \
	  -c uniffi.toml
	@cp target/release/libuni_core.so sdk/python/

test-python: build-python
	@echo "Running Python SDK tests..."
	@python3 sdk/python/test_sdk.py

test-js: build-wasm
	@echo "Running JS/WASM SDK tests..."
	@node sdk/js/test_sdk.js

# Clean all targets
clean:
	@for crate in mate_core uni wasm; do \
		if [ -d $$crate ]; then \
			echo "Cleaning $$crate..."; \
			cd $$crate && $(CARGO) clean && cd ..; \
		fi; \
	done
	@rm -rf sdk/js sdk/python/*.so

# Help
help:
	@echo "Available targets:"
	@echo "  test           - Run all tests"
	@echo "  test-mate_core - Test core crate only"
	@echo "  test-uni       - Test uni (FFI) crate only"
	@echo "  test-wasm      - Test wasm crate only"
	@echo "  build-wasm     - Build WASM package for JS"
	@echo "  build-python   - Build Python SDK"
	@echo "  test-python    - Run Python SDK tests"
	@echo "  test-js        - Run JS/WASM SDK tests"
	@echo "  mate_core      - Alias for test-mate_core"
	@echo "  uni            - Alias for test-uni"
	@echo "  wasm           - Alias for test-wasm"
	@echo "  clean          - Clean all build artifacts"


