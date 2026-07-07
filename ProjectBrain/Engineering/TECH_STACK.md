# Technology Stack

Version: 1.0

Status: Approved

Owner: StorageAI Technologies

Last Updated: 2026-07-06

---

# Philosophy

Every technology used in StorageAI must satisfy the following requirements:

- High Performance
- Long-Term Maintainability
- Cross-Platform Ready
- Open Source
- Production Ready
- Well Documented
- Large Community Support

StorageAI prioritizes stability over trends.

---

# Architecture

Frontend
↓

Desktop Runtime
↓

Core Engine
↓

Database
↓

AI Engine

---

# Frontend

Technology

- React
- TypeScript
- Vite

Reason

- Modern UI
- Component-based architecture
- Fast development
- Excellent ecosystem
- Strong TypeScript support

---

# Desktop Runtime

Technology

Tauri

Reason

- Lightweight
- Small executable size
- Native performance
- Secure architecture
- Uses Rust backend

Alternative Considered

Electron

Reason Rejected

- Higher memory usage
- Larger application size

---

# Core Engine

Technology

Rust

Responsibilities

- Filesystem scanning
- Metadata collection
- Hash generation
- Duplicate detection
- Folder analysis
- Performance-critical operations

Reason

- Excellent performance
- Low memory usage
- Memory safety
- Great concurrency support
- Suitable for scanning millions of files

---

# AI Engine

Technology

Python

Framework

FastAPI

Responsibilities

- AI recommendations
- Metadata analysis
- Natural language search
- Future AI features

Reason

- Best AI ecosystem
- Excellent library support
- Easy integration with AI models

---

# Database

Technology

SQLite

Responsibilities

- Metadata storage
- Scan history
- Duplicate groups
- Dashboard data
- AI cache
- Application settings

Reason

- Embedded database
- Zero configuration
- Reliable
- Fast
- ACID compliant

---

# Hash Algorithm

Primary

SHA-256

Reason

Reliable duplicate verification.

Future

Optional faster pre-filtering before SHA-256 for performance optimization.

---

# Configuration

Technology

TOML

Reason

- Human readable
- Easy versioning
- Common in Rust projects

---

# Logging

Technology

Structured logging

Levels

- INFO
- WARN
- ERROR
- DEBUG

Logs are stored locally.

No user data is transmitted externally.

---

# Testing

Rust

- cargo test

Python

- pytest

Frontend

- Vitest

Future

- Playwright for end-to-end testing

---

# Version Control

Git

Repository

GitHub

Branch Strategy

main

Stable releases

develop

Active development

Feature branches

One branch per feature

---

# Coding Standards

Rust

- cargo fmt
- clippy
- No unsafe code unless justified
- Documentation required

Python

- Black
- Ruff
- Type hints
- Docstrings

TypeScript

- ESLint
- Prettier
- Strict mode enabled

---

# Security

- Local-first architecture
- No automatic cloud uploads
- User data never sold
- AI reads metadata only
- File deletion always requires user confirmation

---

# Future Technologies

Potential future integrations

- ONNX Runtime
- Local LLM support
- Vector search
- Cloud synchronization
- NAS support
- Mobile companion application

These are not part of Version 1.0.

---

# Technology Decisions

| Component | Technology |
|-----------|------------|
| UI | React |
| Language (UI) | TypeScript |
| Build Tool | Vite |
| Desktop Runtime | Tauri |
| Core Engine | Rust |
| AI Engine | Python + FastAPI |
| Database | SQLite |
| Configuration | TOML |
| Logging | Structured Logs |
| Version Control | Git |
| Repository | GitHub |
| Testing | cargo test, pytest, Vitest |
| Hashing | SHA-256 |

---

# Technology Principles

1. Performance over hype.
2. Simplicity over unnecessary complexity.
3. Local-first by default.
4. AI enhances the product but is never required for core functionality.
5. Every technology must have a clear purpose.
6. Prefer mature, production-proven tools.
7. Minimize external dependencies.

---

# Repository Layout

## Application

- React + TypeScript frontend
- Tauri desktop runtime
- Rust backend
- Python AI service (future)

---

## Documentation

Permanent documentation

- ProjectBrain/

Active engineering

- Engineering/

Project planning

- Planning/

---

## Repository Principles

- Keep one source of truth.
- Avoid duplicate implementations.
- Prefer modular design.
- Architecture is frozen for Version 1.