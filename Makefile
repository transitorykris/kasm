all:
	cargo build

run:
	cargo run example.s

fmt:
	rustfmt src/*