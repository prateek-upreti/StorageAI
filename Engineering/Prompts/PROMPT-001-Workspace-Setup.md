# PROMPT-001: Initialize StorageAI Backend

Version: 2.0

Status: Active

---

## Role

You are a Senior Rust Engineer working on StorageAI.

Follow the repository exactly.

Do not redesign the architecture.

Do not create additional folders.

Do not introduce new frameworks.

---

## Objective

Initialize the Rust backend inside the existing Tauri application.

The repository structure is already created.

Only initialize the Rust backend modules.

---

## Repository Structure

apps/
└── desktop/
    └── src-tauri/
        └── src/
            ├── main.rs
            ├── lib.rs
            ├── scanner/
            ├── metadata/
            ├── database/
            ├── hashing/
            ├── duplicate/
            ├── organization/
            ├── rollback/
            ├── events/
            ├── config/
            ├── common/
            └── models/

---

## Requirements

1. Create lib.rs.

2. Register every module.

3. Every module must compile.

4. No business logic.

5. No placeholder TODOs.

6. No UI changes.

7. No database implementation.

8. No scanning implementation.

---

## Deliverables

Create only:

- lib.rs
- mod.rs files for every module

Nothing else.

---

## Code Quality Requirements

- Rust 2024 edition.
- Idiomatic Rust.
- No unwrap().
- No expect().
- No warnings.
- Public modules documented.
- Small, maintainable code.

---

## Definition of Done

cargo build succeeds.

cargo test succeeds.

Zero warnings.

Return only the requested files.