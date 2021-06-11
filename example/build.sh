# Requires: cargo install -f wasm-bindgen-cli

RUSTFLAGS=--cfg=web_sys_unstable_apis cargo build --target wasm32-unknown-unknown
wasm-bindgen --out-dir target/generated-wasm32 --web target/wasm32-unknown-unknown/debug/vesta_example.wasm
