# Makefile
CARGO = cargo
TEST_FLAGS = --verbose --color=always
CRATES = core uni wasm

.PHONY: test $(CRATES) test-all clean help

# Main test target
test: $(addprefix test-,$(CRATES))

# Dynamic targets for each crate
test-core:
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

# Clean all targets
clean:
	@for crate in $(CRATES); do \
		echo "Cleaning $$crate..."; \
		cd $$crate && $(CARGO) clean && cd ..; \
	done

# Help
help:
	@echo "Available targets:"
	@echo "  test       - Run all tests"
	@echo "  test-core  - Test core crate only"
	@echo "  test-uni   - Test uni (FFI) crate only"
	@echo "  test-wasm  - Test wasm crate only"
	@echo "  core       - Alias for test-core"
	@echo "  uni        - Alias for test-uni"
	@echo "  wasm       - Alias for test-wasm"
	@echo "  clean      - Clean all build artifacts"

buin:
	cd uni_core
	cargo build -p uni_core --release
	cargo run --bin uniffi-bindgen generate \
	  --library ./target/release/libuni_core.so \
	  --crate uni_core \
	  --language python \
	  --out-dir sdk/python \
	  -c ./uniffi.toml
	cp -R target/release/libuni_core.so sdk/python/


