OUTPUT_FILE = sinusoidal
GENERATED_FORMAT = ppm
RENDERED_FORMAT = png
build:
	cargo build
run:
	cargo run > $(OUTPUT_FILE).$(GENERATED_FORMAT)
	convert $(OUTPUT_FILE).$(GENERATED_FORMAT) $(OUTPUT_FILE).$(RENDERED_FORMAT)
	xdg-open $(OUTPUT_FILE).$(RENDERED_FORMAT)
clean:
	cargo clean
docs:
	cargo doc
fmt:
	cargo fmt