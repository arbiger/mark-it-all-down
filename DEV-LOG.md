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

## 2026-10-04 — v1/v2 merge and GitHub publication

### Goal

Consolidate Mark-It-Down v1 and Mark-It-All-Down v2 into one maintained
project, and make the v1 hybrid PDF engine durable off-machine.

### Correction to a standing assumption

v1 was assumed to be the text-only predecessor. It is not. v1 holds the PDF
quality core: hybrid PDF routing (`HybridConversion.swift`), per-page OCR
classification that refuses to emit partial Markdown, a private pinned Python
runtime, 41 Swift tests, and a measured ~10.4x median speedup on native-text
PDFs. The merge preserves it as the reference for work v2 has not ported.

### Decisions

- v1 stays a separate working repository; it is not modified or deprecated.
- v2 planning documents moved from v1 into `docs/`, so the public v1 repo
  stays coherent about v1.
- v1 source is copied (not moved) into `v1-swift/`, keeping v1 self-contained.
- Build artifacts were moved to Trash rather than deleted, and the Windows
  portable bundle in `dist/windows/` was deliberately preserved.

### Changed artifacts

- `v1-swift/` — archived v1 SwiftUI source, tests, PDF helper contract,
  benchmark harness, and third-party notices, plus a README explaining the
  quality gap it represents.
- `docs/history/` — v1 design specs and plans, the hybrid PDF baseline, the
  2026-08-20 PDF/OCR plan, and the now-resolved Tauri shell blocker note.
- `docs/plans/`, `docs/benchmarks/` — v2 product plan, evidence spec, corpus
  inventory, and pilot blocker, relocated from the v1 repository.
- `.gitignore`, `README.md` — repository scaffolding and merged product
  documentation.

### Cleanup

Total on-disk footprint fell from 7.4 GB to 719 MB. Trashed: v1 `.build/`
(451 MB) and `Mark-It-Down.app/`, v2 `target/` (339 MB) and
`src-tauri/target/` (5.9 GB). Retained: v1 `dist/windows/` portable bundle,
`Tests/ManualFixtures/`, and the v1 repository itself.

### Verification

```text
cargo test    # 4 passed, exit 0, cold rebuild after target/ removal
```

The v1 hybrid engine work was committed as `2729685` and pushed to
`github.com/arbiger/mark-it-down` after a secret scan found no credentials and
confirmed `Tests/ManualFixtures/` stays ignored.

This project was initialized as `github.com/arbiger/mark-it-all-down` with
initial commit `a94601f`.

### Unresolved risks

- The restored DMG (SHA-256 `21cb3777...`) predates the merge; the merge added
  no code changes to the app, so the binary is still current. A rebuild is
  required to bundle anything further.
- `cargo-tauri` lives in `~/.cargo/bin`, which is not on the default shell
  PATH in this environment; invoke it by absolute path or extend PATH.
- The Codex workspace root `/Users/george/Documents/ChatGPT/Side Projects`
  disappeared during this session. It is unrelated to these repositories.

### Next step

Port v1's per-page OCR classification into the Rust core so that PDFs needing
OCR are refused up front instead of falling through to the renderer path.

## 2026-10-04 — v1 consolidated into v2; licensing established

### Goal

Reduce the two project folders to one, and put the licensing on a correct
footing rather than leaving a public repository with no license at all.

### License decision

The question raised was whether MIT is appropriate given the project builds on
other people's libraries. Verified on 2026-10-04:

- v1's own license was already MIT, Copyright (c) 2026 ycgeorgeli.
- Microsoft MarkItDown 0.1.7 — MIT.
- Firecrawl PDF Inspector 0.2.6 — MIT.
- `zip` 2.4.2, `quick-xml` 0.37.5, `pdf-extract` 0.7.12 — all MIT.
- Tauri and its plugins — MIT OR Apache-2.0.

Every direct dependency is permissive. Permissive licenses do not constrain
this project's own license; they require only that their notices be preserved.
MIT was therefore adopted to match v1 and the ecosystem, with
`THIRD_PARTY_NOTICES.md` recording each dependency and the optional external
tools (Tesseract, Poppler, MuPDF, LibreOffice) that are invoked locally but
never bundled. Poppler is GPL-2.0-or-later and MuPDF is AGPL-3.0-or-later;
because they are executed as separate local programs rather than linked or
redistributed, their copyleft does not extend to this project. Bundling any of
them in a future release would change that and needs a fresh review.

### Changes

- Added `LICENSE` (MIT) and `THIRD_PARTY_NOTICES.md`.
- Set `license` and `authors` in `src-tauri/Cargo.toml` (were empty placeholders).
- Moved v1 build assets, scripts, and app icons into `v1-swift/`.
- Moved the v1 Windows portable bundle and client manual fixtures into
  `v1-artifacts/`, which is gitignored so the 714 MB and the client document
  are never published.
- Moved v1 docs index and SDD working history into `docs/history/`.
- Retired the `Mark-It-Down` folder to Trash after confirming every file was
  represented in v2 or in the v1 GitHub history.

### Verification

```text
cargo test    # 4 passed, exit 0
git check-ignore v1-artifacts/...   # bulk and fixtures correctly ignored
```

Staged content for the consolidation commit was 24 files / 1.3 MB, with no
build output, DMG, Windows bundle, or client fixture included.

### Result

One project folder remains: `Mark-It-All-Down` (996 MB on disk, of which
714 MB is gitignored v1 artifacts and 267 MB is regenerable build cache;
tracked content is roughly 2.5 MB). The v1 folder is in Trash as
`mark-it-down-v1-retired-230344` and remains recoverable. Its repository and
full 33-commit history stay at github.com/arbiger/mark-it-down.

### Next step

Unchanged: port v1's per-page OCR classification into the Rust core.
