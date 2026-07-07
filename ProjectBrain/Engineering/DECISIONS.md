# Architecture Decision Records (ADR)

Version: 1.0

Status: Active

---

# ADR-0001

Title

Local-First Architecture

Status

Accepted

Reason

- Privacy
- Offline support
- Better performance

---

# ADR-0002

Title

Metadata Only

Status

Accepted

Decision

StorageAI never stores user file contents.

Only metadata.

---

# ADR-0003

Title

Dashboard First

Status

Accepted

Decision

The Dashboard is the application's home screen.

The AI assistant is accessed from the dashboard.

---

# ADR-0004

Title

Duplicate Verification

Status

Accepted

Decision

Duplicate files are confirmed only after cryptographic hash verification.

File names are never sufficient.

---

# ADR-0005

Title

Folder Verification

Status

Accepted

Decision

Folders are never considered duplicates based on their names.

A folder is a duplicate only when every child file has been verified.

---

# ADR-0006

Title

AI Architecture

Status

Accepted

Decision

AI reads metadata only.

AI never reads raw user files automatically.

---

# ADR-0007

Title

Technology Stack

Status

Accepted

Decision

Frontend: React + TypeScript

Desktop Runtime: Tauri

Core Engine: Rust

AI Engine: Python + FastAPI

Database: SQLite

---

# ADR-0008

Title

Module Architecture

Status

Accepted

Decision

StorageAI is built as independent engines.

Examples

- Scanner Engine
- Metadata Engine
- Hash Engine
- Duplicate Engine
- Dashboard Engine
- AI Engine

---

# ADR-0009

Title

Safe Delete Policy

Status

Accepted

Decision

StorageAI never deletes files automatically.

User confirmation is always required.

Rollback support is mandatory.

---

# ADR-0010

Title

Version 1 Scope

Status

Accepted

Decision

Version 1 focuses on:

- Scanning
- Metadata
- Duplicate Detection
- Dashboard
- Rollback

Cloud synchronization, plugins, enterprise features, and collaboration are outside Version 1.

---

# ADR-0011

Title

Communication Between Modules

Status

Accepted

Decision

Modules communicate through well-defined interfaces and shared domain models.

Business logic must remain inside the owning engine.

---

# ADR-0012

Title

Quality Standard

Status

Accepted

Decision

Every module must include:

- Documentation
- Tests
- Benchmarks
- Logging
- Error handling
- Review before merge

---

# Future ADRs

Every important engineering decision must receive an ADR number before implementation.