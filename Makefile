dev:
	cargo run -F fast-compile

release:
	cargo build --release -F release
	cp target/release/rustsweeper .

clean:
	cargo clean
	rm web/rustsweeper.js
	rm web/rustsweeper_bg.wasm
	rm rustsweeper

web-release:
	cargo build --profile wasm-release --target wasm32-unknown-unknown -F release
	~/.cargo/bin/wasm-bindgen --no-typescript --out-dir web --out-name rustsweeper --target web target/wasm32-unknown-unknown/wasm-release/rustsweeper.wasm

deps:
	rustup target install wasm32-unknown-unknown
	cargo install wasm-bindgen-cli