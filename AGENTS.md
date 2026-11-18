# Repository instructions

- Keep each explicit Rust type or language feature in its own focused file.
- Follow SOLID and keep packages independently consumable.
- Do not introduce traits or interface abstractions unless a caller requires them.
- Preserve serialized field names, enum values, and validation for published contracts.
- Use `develop` for changes and pull requests to `main`.
- Keep all package versions aligned with the coordinated release version.
