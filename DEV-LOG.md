# Development Log

Append-only implementation record. Add a dated entry for each substantial
change; do not rewrite earlier entries.

## 2026-09-06 — Packaged-app PDF runtime fix and handoff

### Goal

Make the Mark-It-All-Down v2 Tauri app usable when installed from a DMG on a
different Mac, without binding it to a service or cloud runtime.

### Decisions

- Keep the app local-first and single-process from the user's perspective.
- Use in-process Rust PDF extraction first, so normal text PDFs do not require
  Homebrew or a user-installed `pdftotext`.
- Keep external OCR/rendering tools optional and detect them through explicit
  executable paths; never evaluate source paths through a shell.
- Preserve the v1 Mark-It-Down project as a separate, untouched application.

### Changed artifacts

- `Cargo.toml` / `Cargo.lock`: added `pdf-extract` alongside the existing
  `zip` and `quick-xml` extraction dependencies.
- `src/lib.rs`: normal PDFs use in-process extraction before optional Poppler;
  scanned PDFs use the existing render-plus-Tesseract fallback; tool lookup
  checks the launcher's restricted PATH and standard macOS locations.
- `src-tauri/src/lib.rs`: native queue and result commands expose actionable
  conversion errors and path classification for drag/drop.
- `web/index.html` / `web/app.js`: native drag/drop, queue status spacing, and
  local-conversion wording were retained in the packaged UI.
- `README.md`: updated adapter and runtime-prerequisite documentation.
- `HANDOFF.md`: current operator instructions and known boundaries.

### Verification evidence

The following checks passed on 2026-09-06:

```text
cargo fmt -- --check
cargo test                         # 4 passed
cargo build
cargo test --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml
node --check web/app.js
cargo tauri build --debug --bundles dmg
```

The final Apple Silicon DMG was generated at:

`src-tauri/target/debug/bundle/dmg/Mark-It-All-Down_0.1.0_aarch64.dmg`

SHA-256:

`21cb377791efc400e3fb7cdeea4e7a7e4fe75a55cf7b94a0903391f468b29a11`

Detailed run record: `docs/ai-runs/2026-09-06/2148-packaged-pdf-runtime-fix.md`.

### Unresolved risks

- The artifact is `aarch64` and is intended for Apple Silicon Macs; no Intel
  (`x86_64`) or Windows artifact has been built in this run.
- Image OCR and scanned-PDF OCR still require a local Tesseract installation;
  normal text PDFs do not require Poppler after this fix.
- LibreOffice remains an optional runtime for legacy binary Office formats.
- Extraction is text-oriented and does not guarantee visual layout, tables, or
  formulas will round-trip exactly.
- This is local verification, not independent testing on a second Mac.

### Next step

Install the fresh DMG on the target Apple Silicon Mac and convert a normal
text PDF. If scanned PDF or image OCR is required there, install Tesseract and
the PDF renderer, or approve a future bundled-runtime packaging pass.
