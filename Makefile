OUTPUT_FILE = sinusoidal.ppm
build:
	cargo build
run:
	cargo run > $(OUTPUT_FILE)
	xdg-open $(OUTPUT_FILE)
clean:
	cargo clean
docs:
	cargo doc
fmt:
	cargo fmt