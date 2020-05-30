all:
	rustfmt src/*.rs src/bin/*.rs
	cargo build

run:
	rustfmt src/*.rs src/bin/*.rs
	cargo run example.s

fmt:
	rustfmt src/*.rs src/bin/*.rs

dump:
	hexdump -C a.out