add more packageto build
```bash
rustup target add x86_64-apple-darwin
rustup component add rust-src --toolchain nightly-2024-12-31-x86_64-unknown-linux-gnu

cargo build --target x86_64-apple-darwin -Zbuild-std

```