run:
	cargo run

release:
	cargo build --release

wasm-release:
	cargo build --target wasm32-unknown-unknown --profile wasm-release \
	&& wasm-bindgen --target web --no-typescript --out-dir ./pkg --out-name trix_blasting ./target/wasm32-unknown-unknown/wasm-release/trix_blasting.wasm
