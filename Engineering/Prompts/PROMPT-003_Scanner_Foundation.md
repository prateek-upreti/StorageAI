# PROMPT-003: Scanner Foundation

Version: 2.0

Status: Active

Owner: StorageAI Technologies

Last Updated: 2026-07-07

---

# Role

You are a Senior Rust Software Engineer working on StorageAI.

StorageAI is a professional desktop application built using:

- Rust
- Tauri
- React
- TypeScript
- SQLite

You are working inside an existing repository.

Do NOT redesign the architecture.

Do NOT rename folders.

Do NOT introduce new modules.

Do NOT change project structure.

Follow the repository exactly as it exists.

---

# Before Writing Code

First inspect the existing repository.

Understand:

- Current folder structure
- Existing Rust modules
- Cargo configuration
- Existing coding style

Generate code that fits naturally into the current repository.

---

# Objective

Implement the Scanner Foundation.

Generate ONLY the following files:

apps/desktop/src-tauri/src/scanner/

- mod.rs
- errors.rs
- types.rs
- progress.rs

Do NOT generate any other files.

Do NOT modify existing files.

---

# Project Context

StorageAI is a local-first desktop application.

The Scanner module is responsible only for discovering files and folders.

It does NOT:

- Hash files
- Read metadata
- Store data
- Delete files
- Organize files
- Interact with the database
- Communicate with the UI

Those responsibilities belong to other modules.

---

# Scanner Module Responsibilities

The scanner foundation provides:

- Common scanner types
- Scanner error definitions
- Progress reporting structures
- Public scanner module exports

No traversal logic yet.

No filesystem implementation.

---

# File Requirements

## mod.rs

Responsibilities

- Register scanner modules
- Export public scanner API
- Keep imports clean
- Keep module lightweight

No implementation logic.

---

## errors.rs

Create a production-ready ScannerError enum.

Support at minimum:

- PermissionDenied
- PathNotFound
- InvalidPath
- SymbolicLinkLoop
- IoError
- Cancelled
- Unknown

Requirements

Implement:

- std::fmt::Display
- std::error::Error
- From<std::io::Error>

Error messages should be clear.

Avoid generic wording.

---

## types.rs

Define shared scanner data structures.

Examples include:

- ScanTarget
- ScanStatistics
- ScanOptions

Only include structures required by the scanner foundation.

No filesystem traversal.

No metadata.

No hashing.

No database models.

Public structures must include Rust documentation comments.

---

## progress.rs

Define scanner progress models.

Support:

- Files scanned
- Folders scanned
- Current path
- Elapsed duration
- Scan state

Represent scan state using an enum.

No UI-specific code.

No event bus implementation.

Only shared progress models.

---

# Coding Standards

Rust Edition 2024.

Follow idiomatic Rust.

Public APIs require documentation comments (///).

Keep functions short.

Keep modules cohesive.

Prefer explicit code over clever code.

Avoid unnecessary allocations.

Borrow instead of cloning whenever practical.

No unsafe code.

No unwrap().

No expect().

No panic!() unless completely justified.

No TODO comments.

No placeholder implementations.

No dead code.

---

# Performance Requirements

The scanner will eventually process millions of files.

Design with scalability in mind.

Avoid designs that require loading the entire filesystem into memory.

Do not optimize prematurely, but avoid obvious inefficiencies.

---

# Security Requirements

StorageAI is local-first.

Never expose user data.

Never log file contents.

Do not implement deletion.

Do not implement modification.

Read-only architecture.

---

# Constraints

Do NOT implement:

- Filesystem walking
- Metadata extraction
- Database integration
- Hashing
- Duplicate detection
- Organization
- Rollback
- UI integration
- AI functionality

Foundation only.

---

# Expected Output

Return exactly these four files:

apps/desktop/src-tauri/src/scanner/mod.rs

apps/desktop/src-tauri/src/scanner/errors.rs

apps/desktop/src-tauri/src/scanner/types.rs

apps/desktop/src-tauri/src/scanner/progress.rs

Do not include explanations.

Do not include architecture suggestions.

Do not generate additional files.

Return production-ready Rust code only.

---

# Self Review

Before returning the code, verify:

✓ Compiles

✓ Idiomatic Rust

✓ No warnings

✓ Public APIs documented

✓ No unwrap()

✓ No expect()

✓ No placeholder code

✓ Matches existing repository

If any item fails, fix it before returning the final result.