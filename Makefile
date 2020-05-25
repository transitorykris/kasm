all:
	rustfmt src/*
	cargo build

run:
	rustfmt src/*
	cargo run example.s

fmt:
	rustfmt src/*