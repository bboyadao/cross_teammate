# Makefile
CARGO = cargo
WASM_PACK = wasm-pack
TEST_FLAGS = --verbose --color=always
CRATES = mate_core uni wasm

.PHONY: test $(CRATES) test-all clean help build-wasm build-python

# Main test target
test: $(addprefix test-,$(CRATES))

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
	@cd uni && $(CARGO) build --release
	@$(CARGO) run -p uni_core --bin uniffi-bindgen generate \
	  --library ../target/release/libteammate.so \
	  --crate uni_core \
	  --language python \
	  --out-dir ../sdk/python \
	  -c ./uniffi.toml
	@cp -R target/release/libteammate.so sdk/python/

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
	@echo "  mate_core      - Alias for test-mate_core"
	@echo "  uni            - Alias for test-uni"
	@echo "  wasm           - Alias for test-wasm"
	@echo "  clean          - Clean all build artifacts"


