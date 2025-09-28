bpy:
	cargo build --release
	cargo run --bin uniffi-bindgen generate --library target/release/libteammate.so --language python --out-dir sdk/python
	cp -R target/release/libteammate.so sdk/python/

t:
	cargo test --color=always --package TEAM --test teammate_test --no-fail-fast -- --exact --show-output