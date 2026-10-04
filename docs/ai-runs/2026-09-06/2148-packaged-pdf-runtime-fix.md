# AI Run Record — Packaged PDF runtime fix

- Date/time: 2026-09-06, Asia/Taipei (CST)
- Task: make the installed Tauri DMG convert normal PDFs on a clean Mac
- Agent surface: Codex desktop, local workspace
- Coordinator model: not exposed in this run
- Implementation/verification agent: Luna Max (`gpt-5.6-luna`), same worker
- Session ID: not exposed
- Usage/cost: not exposed for this run

## Scope

Replace the packaged app's hard dependency on a Finder-inherited `PATH` entry
for `pdftotext`, while preserving local-only behavior and existing OCR and
Office adapters.

## Decisions

- Use `pdf-extract` in-process before external tools for normal PDFs.
- Retain external renderer/Tesseract detection only for scanned-PDF OCR.
- Keep executable resolution explicit and shell-free.
- Do not modify v1, add a service, download models, or publish/sign artifacts.

## Changed files

- `Cargo.toml`
- `Cargo.lock`
- `src/lib.rs`
- `web/index.html`
- `README.md`
- `DEV-LOG.md`
- `HANDOFF.md`

## Verification

Passed on the local macOS host:

```text
cargo fmt -- --check
cargo test                         # 4 passed
cargo build
cargo test --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml
node --check web/app.js
cargo tauri build --debug --bundles dmg
```

Artifact:

`src-tauri/target/debug/bundle/dmg/Mark-It-All-Down_0.1.0_aarch64.dmg`

SHA-256:

`21cb377791efc400e3fb7cdeea4e7a7e4fe75a55cf7b94a0903391f468b29a11`

## Review and limitations

- Review type: coordinator red-blue review, not independent review.
- Verification type: separate same-worker verification, not independent
  testing.
- The DMG is Apple Silicon only and has not been runtime-tested on the second
  Mac in this run.
- Scanned-PDF/image OCR still depends on local Tesseract and a renderer.
- Legacy Office conversion still depends on local LibreOffice/soffice.
