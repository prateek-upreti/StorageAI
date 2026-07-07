# StorageAI AI Context

Version: 1.0

Status: Active

Last Updated: 2026-07-07

Owner: StorageAI Technologies

---

# Purpose

This file is the first document every AI assistant (ChatGPT, Claude, Codex, etc.) must read before making changes to the StorageAI repository.

It summarizes the project's architecture, engineering rules, current implementation status, and development workflow.

This file does **not** replace the official documentation inside `ProjectBrain/`. It is a concise working context to reduce mistakes and maintain consistency.

---

# Project Goal

StorageAI is a local-first desktop application that helps users:

- Analyze storage usage
- Scan drives
- Detect duplicate files
- Organize files safely
- Provide intelligent recommendations using AI

Core functionality must work completely offline.

AI features enhance the product but are never required for core functionality.

---

# Technology Stack

## Frontend

- React
- TypeScript
- Vite

## Desktop Runtime

- Tauri v2

## Backend

- Rust (Edition 2024)

## AI

- Python
- FastAPI

## Database

- SQLite

---

# Repository Structure

StorageAI/

- apps/
  - desktop/
    - src/
    - src-tauri/

- Engineering/

- ProjectBrain/

- AI_CONTEXT.md

---

# Project Rules

AI assistants must follow these rules:

1. Do not redesign the architecture.
2. Do not rename folders.
3. Do not move files without approval.
4. Do not introduce new dependencies unless approved.
5. Keep every change small and focused.
6. Keep the project compiling after every change.
7. Avoid placeholder implementations.
8. Prefer reusable components over duplicated code.
9. Keep one responsibility per module.
10. Follow existing engineering standards.

---

# Development Workflow

Every feature follows this process:

Design

↓

Implementation

↓

Code Review

↓

Compilation

↓

Testing

↓

Merge

No feature is considered complete until the project builds successfully.

---

# Current Project Status

Completed

- Project initialization
- Rust environment
- React environment
- Tauri setup
- React ↔ Rust communication
- Backend commands module
- Frontend constants
- Backend constants
- Reusable Button component
- Initial application shell

In Progress

- Scanner Engine

Planned

- Metadata Engine
- Database Layer
- Duplicate Detection
- Dashboard
- AI Recommendation Engine

---

# Coding Guidelines

Rust

- Prefer Result over panic.
- Avoid unwrap().
- Avoid expect() except during application startup.
- Document public APIs.
- Keep modules cohesive.

TypeScript

- Use strict typing.
- Prefer reusable components.
- Avoid duplicated logic.

General

- Every file should have a clear responsibility.
- Every feature should compile before continuing.

---

# Documentation

Project documentation lives in:

ProjectBrain/

Engineering prompts live in:

Engineering/Prompts/

This AI context file is a quick reference only.

---

# Long-Term Vision

StorageAI should become a professional, production-ready desktop application capable of scanning millions of files efficiently while remaining local-first, secure, and maintainable.