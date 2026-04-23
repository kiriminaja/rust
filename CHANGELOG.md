# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Bug Fixes

- Update authors and repository information in Cargo.toml; correct installation section in README.md

### CI

- Bump MSRV to 1.88 (wiremock 0.6.5 requires let-chains)
- Bump MSRV to 1.86 to satisfy transitive deps (icu_*, indexmap edition2024)
- Add GitHub Actions test workflow (MSRV 1.75 + stable)

### Documentation

- Reformat config options table alignment

### Features

- *(utils)* Add multi-item volumetric calculator + revamp README
- Add coverage area and pricing types for express and instant services

### Miscellaneous

- Add changelog target (git-cliff) + cliff.toml config
- Add Makefile and release workflow (cargo publish on v* tag)

### Styling

- Derive Default for Env (clippy::derivable_impls)
- Apply rustfmt across src/tests/examples


