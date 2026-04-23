CARGO := cargo

.PHONY: install build test fmt fmt-check lint clean publish-dry publish release

install:
	$(CARGO) fetch

build:
	$(CARGO) build --all-features

test:
	$(CARGO) test --all-features

fmt:
	$(CARGO) fmt --all

fmt-check:
	$(CARGO) fmt --all -- --check

lint:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

clean:
	$(CARGO) clean

publish-dry:
	$(CARGO) publish --dry-run

publish:
	$(CARGO) publish

release: fmt-check lint test publish
