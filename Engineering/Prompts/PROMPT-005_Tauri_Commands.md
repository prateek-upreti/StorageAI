# PROMPT-005: Tauri Commands Foundation

Version: 1.0

Status: Active

Owner: StorageAI Technologies

Last Updated: 2026-07-07

---

## Role

You are a Senior Rust and Tauri Engineer.

You are working inside the existing StorageAI repository.

Do not redesign the architecture.

Do not rename folders.

Do not modify Cargo.toml unless required.

---

## Objective

Create the initial Tauri command foundation.

Generate only the code necessary to expose backend commands to the frontend.

---

## Requirements

Implement the following commands:

### app_version

Returns:

```
StorageAI 0.1.0
```

---

### health_check

Returns:

```
OK
```

---

Register both commands in the Tauri builder.

---

## Constraints

Do not implement:

- Scanner
- Database
- Hashing
- Metadata
- AI
- File operations
- Settings

No business logic.

Only command registration.

---

## Code Quality

Rust Edition 2024.

Idiomatic Rust.

Public APIs documented.

No unwrap().

No expect() except application startup.

No warnings.

---

## Output

Return only the files that require modification.

Do not generate explanations.