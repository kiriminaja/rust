CARGO := cargo
GIT_CLIFF := git-cliff

.PHONY: install build test fmt fmt-check lint clean changelog changelog-unreleased publish-dry publish release

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

# ---------------------------------------------------------------------------
# Changelog (powered by git-cliff — https://git-cliff.org)
# Install once: cargo install git-cliff
# ---------------------------------------------------------------------------

changelog:
	@command -v $(GIT_CLIFF) >/dev/null 2>&1 || { \
		echo "❌ git-cliff not found. Install with: cargo install git-cliff"; exit 1; }
	@echo "📝 Generating CHANGELOG.md …"
	@$(GIT_CLIFF) -o CHANGELOG.md
	@echo "✅ CHANGELOG.md updated"

changelog-unreleased:
	@command -v $(GIT_CLIFF) >/dev/null 2>&1 || { \
		echo "❌ git-cliff not found. Install with: cargo install git-cliff"; exit 1; }
	@$(GIT_CLIFF) --unreleased --strip header

publish-dry:
	$(CARGO) publish --dry-run

publish:
	$(CARGO) publish

release: fmt-check lint test changelog publish
