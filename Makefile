all:
	rustfmt src/*.rs src/bin/*.rs
	cargo clippy
	cargo build

run:
	cargo run example.s

fmt:
	rustfmt src/*.rs src/bin/*.rs

test:
	cargo test

dump:
	hexdump -C a.out

clippy:
	cargo clippy