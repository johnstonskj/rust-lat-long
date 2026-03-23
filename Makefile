CARGO = cargo
SCOPE = --workspace
OUTPUT = --quiet
# OUTPUT = --verbose

all : format clippy test docs

format : check
	cargo fmt $(OUTPUT)

check :
	cargo check $(SCOPE) $(OUTPUT)

clippy : format
	cargo clippy $(SCOPE) $(OUTPUT)

build : check
	cargo build $(SCOPE) $(OUTPUT)
	cargo build $(SCOPE) --release $(OUTPUT)
	cargo build $(SCOPE) --all-features $(OUTPUT)
	cargo build $(SCOPE) --all-features --release $(OUTPUT)
	cargo build $(SCOPE) --no-default-features $(OUTPUT)
	cargo build $(SCOPE) --no-default-features --release $(OUTPUT)

test : build
	cargo test $(SCOPE) $(OUTPUT)
	cargo test $(SCOPE) --all-features $(OUTPUT)
	cargo test $(SCOPE) --no-default-features $(OUTPUT)
	cargo test $(SCOPE) --no-default-features --features "urn" $(OUTPUT)
	cargo test $(SCOPE) --no-default-features --features "serde" $(OUTPUT)
	cargo test $(SCOPE) --no-default-features --features "geojson" $(OUTPUT)
	cargo test $(SCOPE) --no-default-features --features "urn,serde,geojson" $(OUTPUT)

coverage : build
	cargo tarpaulin $(SCOPE)

docs : check
	cargo doc $(SCOPE) --all-features --no-deps $(OUTPUT)

publish : clippy test docs
	cargo publish --dry-run --allow-dirty $(OUTPUT)
