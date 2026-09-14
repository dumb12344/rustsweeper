dev:
	cargo run -F fast-compile

release:
	cargo build --release -F release
	cp target/release/rustsweeper .