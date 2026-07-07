---

## Code Quality Requirements

- Rust 2024 edition.
- No `unwrap()` or `expect()` unless fully justified.
- Public APIs must include Rust documentation comments (`///`).
- Keep functions small and focused.
- Follow idiomatic Rust naming conventions.
- Prefer exhaustive pattern matching.
- Avoid unnecessary heap allocations.
- Code must compile without warnings.
- Return only the contents of `errors.rs`.