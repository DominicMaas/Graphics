SET RUSTFLAGS=--cfg=web_sys_unstable_apis 

cargo build --target wasm32-unknown-unknown

wasm-bindgen --out-dir web --web target/wasm32-unknown-unknown/debug/vesta_example.wasm
