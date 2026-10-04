# Mark-It-All-Down v2 — Current Handoff

## Read first

1. `README.md`
2. `DEV-LOG.md`
3. `/Users/george/Documents/Georges/01 🎯 Projects/Mark-It-Down/docs/plans/2026-08-29-mark-it-all-down-v2-product-plan.md`
4. `/Users/george/Documents/Georges/01 🎯 Projects/Mark-It-Down/docs/plans/2026-08-29-mark-it-all-down-v2-phase-0-evidence-spec.md`

## Current state

Mark-It-All-Down v2 is a local-first Tauri desktop converter. It is a sibling
project and does not modify the original Swift/Python Mark-It-Down v1 app.
The current build has a native queue, file/folder pickers and drag/drop,
same-folder or chosen-folder output routing, cancellation, collision-safe
Markdown names, atomic writes, and Reveal output behavior.

### Conversion adapters

- Text/markup: `.txt`, `.md`, `.rst`, `.csv`, `.tsv`, `.html`, `.xhtml`
- PDF: in-process Rust extraction first; scanned PDFs fall back to local page
  rendering plus Tesseract OCR
- Images: `.png`, `.jpg`, `.jpeg`, `.tif`, `.tiff`, `.bmp`, `.webp` through
  local Tesseract OCR
- OOXML: `.docx`, `.docm`, `.xlsx`, `.xlsm`, `.pptx`, `.pptm`
- ODF: `.odt`, `.ods`, `.odp`
- Legacy Office: `.doc`, `.dot`, `.xls`, `.xlt`, `.ppt`, `.pot` through a
  detected local LibreOffice/soffice installation

The app reports actionable errors when optional tools are missing. It does not
write fabricated placeholder Markdown for failed conversions.

## Install / run

The current distributable is Apple Silicon only:

`src-tauri/target/debug/bundle/dmg/Mark-It-All-Down_0.1.0_aarch64.dmg`

SHA-256:

`21cb377791efc400e3fb7cdeea4e7a7e4fe75a55cf7b94a0903391f468b29a11`

Double-click the DMG and drag the app into Applications. Replace older
installations before testing the packaged PDF fix.

For development:

```sh
cd "/Users/george/Documents/Georges/01 🎯 Projects/Mark-It-All-Down"
cargo tauri dev
```

## Runtime prerequisites

Normal text PDFs use the bundled Rust extractor and do not need Homebrew.
Scanned PDFs and image OCR require `tesseract` and either `pdftoppm` or
`mutool`; legacy Office files require LibreOffice/soffice. The app checks PATH
and common macOS absolute locations and reports what it searched.

## Verification completed

On 2026-09-06, these checks passed:

```text
cargo fmt -- --check
cargo test                         # 4 passed
cargo build
cargo test --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml
node --check web/app.js
cargo tauri build --debug --bundles dmg
```

## Known boundaries

- Only an Apple Silicon macOS DMG is currently produced; Intel macOS and
  Windows builds still need their own build/signing runs.
- Optional OCR and LibreOffice runtimes are not embedded in the DMG.
- Office extraction is text-oriented; complex layout, tables, formulas, and
  embedded objects need fixture-based quality review.
- No second-Mac runtime test has been completed in this workspace after the
  latest DMG rebuild; the reported clean-machine failure motivated the
  in-process PDF change.
- The legacy internal function name `stub` remains in the Rust API even though
  supported formats now route to real adapters; rename it in a future cleanup
  only with a compatibility check.

## Next atomic task

Install the fresh DMG on the second Apple Silicon Mac and convert the reported
normal PDF. Record the result and the generated `.md` path. Then test one
scanned PDF and one image to confirm whether the target Mac has the optional
OCR runtimes. Do not delete prior outputs during this check.

## Human gates

- Approve any bundled OCR/runtime packaging decision because it changes DMG
  size, licensing review, and cross-platform distribution.
- Approve Intel/Windows release builds and signing/notarization before sharing
  them outside the local test group.
