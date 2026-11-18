# Nexora Platform Contracts

Shared, serialization-stable Rust types for Nexora services. This repository owns cross-service identifiers, durable job contracts, wake-up message shapes, and the small domain values needed by API and worker packages.

All packages in this repository are released together at version `0.4.0`. API-only request/response models, persistence, transport handlers, analysis engines, and language profiles belong in their owning repositories.

## Repository status

The `0.4.0` extraction is in progress on `develop`. This repository is the source of truth for the shared contracts after that work is merged.

## Development

Use the Rust toolchain declared by the workspace. Run `cargo fmt --all -- --check`, `cargo check --workspace --locked`, `cargo test --workspace --locked`, and `cargo clippy --workspace --all-targets --locked -- -D warnings` before submitting changes.

See [CONTRIBUTING.md](CONTRIBUTING.md), [SECURITY.md](SECURITY.md), and [CHANGELOG.md](CHANGELOG.md).
