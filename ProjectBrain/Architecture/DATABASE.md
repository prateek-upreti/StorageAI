# Database Design

Version: 1.0

Status: Approved

Owner: StorageAI Technologies

Database: SQLite

---

# Philosophy

StorageAI stores metadata only.

The database never stores the actual content of user files.

The database is designed for:

- Fast searching
- Incremental scanning
- Duplicate detection
- Dashboard analytics
- AI recommendations
- Scan history
- Rollback support

---

# Database Tables

## scans

Purpose

Represents every scan performed by StorageAI.

Columns

- id
- scan_name
- root_path
- started_at
- completed_at
- status
- total_files
- total_folders
- total_size
- duration

---

## folders

Purpose

Stores discovered folders.

Columns

- id
- scan_id
- parent_id
- path
- name
- created_time
- modified_time
- file_count
- folder_count

---

## files

Purpose

Stores every discovered file.

Columns

- id
- scan_id
- folder_id
- path
- name
- extension
- size
- created_time
- modified_time
- accessed_time
- attributes
- hash
- state
- duplicate_group_id
- is_deleted
- last_seen

---

## duplicate_groups

Purpose

Groups files that are confirmed duplicates.

Columns

- id
- hash
- total_files
- wasted_space
- verified

---

## settings

Purpose

Application configuration.

Examples

- Theme
- Language
- Scan options
- Hash algorithm

---

## ai_cache

Purpose

Stores AI-generated recommendations.

Columns

- id
- prompt_hash
- response
- created_at
- expires_at

---

## reports

Purpose

Stores generated reports.

Columns

- id
- report_name
- report_type
- created_at
- location

---

# File Lifecycle

Every file moves through the following states.

DISCOVERED

↓

METADATA_READ

↓

HASHED

↓

ANALYZED

↓

DUPLICATE_VERIFIED

↓

ORGANIZED

↓

ARCHIVED

---

# Relationships

One Scan

↓

Many Folders

↓

Many Files

↓

One Duplicate Group

---

# Indexes

Indexes should exist on:

- path
- hash
- extension
- size
- modified_time
- duplicate_group_id

---

# Database Rules

1. Never store file contents.

2. Never delete metadata automatically.

3. Every scan creates history.

4. Duplicate verification requires matching hashes.

5. Rollback information must always remain available.

---

# Future Tables

Not Version 1.0

- tags
- bookmarks
- cloud_accounts
- plugins
- automation_rules
- ai_memory

---

# Design Principles

- Normalize where practical.
- Avoid duplicated metadata.
- Optimize for read performance.
- Support incremental updates.
- Keep migrations backward compatible.