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

t:
	cargo test --color=always --package TEAM --test teammate_test --no-fail-fast -- --exact --show-output