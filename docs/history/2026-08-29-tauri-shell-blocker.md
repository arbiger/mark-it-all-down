# Tauri desktop-shell blocker

**Status:** Blocked pending explicit toolchain setup approval  
**Date:** 2026-08-29

The requested native Tauri shell was not implemented because the current
environment does not provide Tauri tooling:

- `rustc 1.98.0 (88d9e12ae 2026-08-18)`
- `cargo 1.98.0 (797e8a9bc 2026-08-05)`
- `node v26.7.0`
- `cargo tauri --version` → `error: no such command: tauri`
- no `tauri` executable on `PATH`

No package installation, network download, model/OCR/converter setup, or
platform SDK change was attempted. Installing and validating Tauri would add
new dependencies and platform build requirements; this needs a separately
approved setup step. The existing dependency-free browser fallback remains
unchanged and the Rust core still passes its focused tests.

This is a tooling blocker, not an engine or architecture decision. Docling,
Xberg, OCR, cloud/LLM behavior, service binding, and packaging remain deferred.
