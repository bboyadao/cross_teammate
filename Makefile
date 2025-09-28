bpy:
	cargo build --release
	cargo run --bin uniffi-bindgen generate --library target/release/libteammate.so --language python --out-dir sdk/python
	cp -R target/release/libteammate.so sdk/python/