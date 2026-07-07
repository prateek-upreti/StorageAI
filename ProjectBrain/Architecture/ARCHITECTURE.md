# StorageAI Desktop System Architecture

Version: 2.0

Status: Approved

Owner: StorageAI Technologies

Last Updated: 2026-07-07

Purpose:

Define the long-term software architecture of StorageAI.
This document is the single source of truth for the application's architecture.
All implementation must follow this document unless an Architecture Decision Record (ADR) explicitly changes it.

---

# Architecture Principles

StorageAI is designed around the following principles:

- Local-first
- Privacy-first
- Performance-first
- Modular architecture
- Maintainable codebase
- Production-ready engineering
- Scalability without unnecessary complexity

The architecture is frozen for Version 1.0 unless an approved architectural decision requires modification.

---

# High-Level Architecture

```
                 React + TypeScript UI
                         │
                         ▼
                    Tauri Desktop
                         │
               Rust Backend (Core)
                         │
        ┌────────────────┼────────────────┐
        ▼                ▼                ▼
    Scanner        Metadata Engine    Database
                         │
                         ▼
                     SQLite
                         │
                         ▼
               AI Memory Engine (Future)
```

---

# Repository Structure

```
StorageAI/
│
├── apps/
│   └── desktop/
│       ├── src/                  # React UI
│       └── src-tauri/            # Rust backend
│
├── assets/                       # Icons, logos, fonts
├── docs/                         # User documentation
├── Engineering/                  # Active engineering work
├── Planning/                     # Project planning
├── ProjectBrain/                 # Permanent knowledge
├── releases/                     # Release artifacts
├── scripts/                      # Development scripts
├── tests/                        # Automated tests
├── tools/                        # Internal tools
│
├── README.md
├── LICENSE
└── .gitignore
```

---

# Backend Architecture

Rust backend location:

```
apps/
└── desktop/
    └── src-tauri/
        └── src/
```

Current module structure:

```
src/
│
├── main.rs
├── lib.rs
│
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
```

Each module has one responsibility.

Modules communicate through public interfaces.

Business logic must never be duplicated.

---

# Frontend Architecture

Location

```
apps/desktop/src/
```

Responsibilities

- User Interface
- State Management
- User Interaction
- Progress Display
- Dashboard
- Settings

The frontend never scans the filesystem directly.

All filesystem operations occur through the Rust backend.

---

# Core Modules

## Scanner

Responsibilities

- Recursive filesystem traversal
- File discovery
- Folder discovery
- Progress reporting
- Cancellation
- Ignore rules

---

## Metadata

Responsibilities

- Read metadata
- File size
- Dates
- Attributes
- Extension
- MIME type (future)

---

## Database

Technology

SQLite

Responsibilities

- Metadata storage
- Incremental scanning
- Duplicate groups
- Scan history
- Dashboard data
- Application settings

---

## Hashing

Responsibilities

- SHA-256 hashing
- Incremental hashing
- Large file optimization

---

## Duplicate

Responsibilities

- Exact duplicate detection
- Duplicate grouping
- Safe deletion preparation

---

## Organization

Responsibilities

- Folder statistics
- Storage categorization
- Empty folder detection
- Large folder analysis

---

## Rollback

Responsibilities

- Undo delete
- Restore operations
- Recovery history

---

## Events

Responsibilities

- Progress events
- UI notifications
- Scan lifecycle events

---

## Config

Responsibilities

- Application configuration
- User preferences
- Scan configuration

---

## Common

Responsibilities

Shared utilities used by multiple modules.

---

## Models

Responsibilities

Shared data structures.

---

# Data Flow

```
User

↓

Select Drive

↓

Scanner

↓

Metadata

↓

SQLite Database

↓

Hashing

↓

Duplicate Detection

↓

Dashboard

↓

User
```

Future

```
Database

↓

AI Memory Engine

↓

Recommendations

↓

User
```

---

# Development Order

Version 1 milestones

1. Scanner Engine
2. Metadata Engine
3. SQLite Engine
4. Hash Engine
5. Duplicate Engine
6. Organization Engine
7. Dashboard
8. Rollback
9. Professional UI

Future

10. AI Memory Engine
11. Local AI
12. Cloud Sync
13. Mobile Companion

---

# Engineering Rules

Every module must:

- Have one responsibility.
- Compile independently.
- Include documentation.
- Include unit tests.
- Avoid duplicated logic.
- Follow Rust best practices.

---

# Security Principles

StorageAI is local-first.

Rules:

- No automatic cloud uploads.
- No telemetry without user consent.
- No automatic deletion.
- User confirmation required before destructive operations.
- AI processes metadata only unless explicitly authorized.

---

# Performance Goals

Version 1 targets

- Scan millions of files.
- Efficient memory usage.
- Responsive UI during scans.
- Incremental rescans.
- Multi-threaded scanning where appropriate.

---

# Out of Scope (Version 1)

- Cloud synchronization
- Plugin marketplace
- Mobile application
- Enterprise collaboration
- Real-time remote synchronization

---

# Success Criteria

StorageAI Version 1 is complete when:

- Large drives can be scanned reliably.
- Duplicate detection is accurate.
- Rollback works correctly.
- Dashboard provides actionable insights.
- All automated tests pass.
- No critical defects remain.

---

# Related Documents

Architecture decisions:

ProjectBrain/Engineering/DECISIONS.md

Technology stack:

ProjectBrain/Engineering/TECH_STACK.md

Product roadmap:

ProjectBrain/Product/ROADMAP.md

Engineering work:

Engineering/DOCUMENT_INDEX.md