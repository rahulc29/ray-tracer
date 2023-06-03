OUTPUT_FILE = sphere-1
GENERATED_FORMAT = ppm
RENDERED_FORMAT = png
build:
	cargo build --verbose
run:
	cargo run > $(OUTPUT_FILE).$(GENERATED_FORMAT)
	convert $(OUTPUT_FILE).$(GENERATED_FORMAT) $(OUTPUT_FILE).$(RENDERED_FORMAT)
	mv $(OUTPUT_FILE).$(RENDERED_FORMAT) rendered/$(OUTPUT_FILE).$(RENDERED_FORMAT)
	git add rendered/*
	xdg-open rendered/$(OUTPUT_FILE).$(RENDERED_FORMAT)
clean:
	cargo clean
docs:
	cargo doc
fmt:
	cargo fmt
test:
	cargo test --verbose