# Mark-It-All-Down

A local-first desktop converter that turns documents, spreadsheets,
presentations, PDFs, and images into Markdown. Drop one file, many files, or
whole folders; outputs land beside the sources or in a folder you choose.

Built with Tauri and Rust so the same codebase ships on macOS and Windows.
Conversion runs entirely on your machine — no service, no account, no cloud
call.

## What it converts

| Input | How it is handled |
|---|---|
| `.txt` `.md` `.rst` `.csv` `.tsv` `.html` `.xhtml` | Read directly as UTF-8 |
| `.pdf` (text) | In-process Rust extraction — no extra install needed |
| `.pdf` (scanned) | Page rendering plus Tesseract OCR |
| `.png` `.jpg` `.jpeg` `.tif` `.tiff` `.bmp` `.webp` | Tesseract OCR |
| `.docx` `.docm` `.xlsx` `.xlsm` `.pptx` `.pptm` | Native OOXML/XML archive reading |
| `.odt` `.ods` `.odp` | Native ODF archive reading |
| `.doc` `.dot` `.xls` `.xlt` `.ppt` `.pot` | Detected local LibreOffice/soffice |

Normal text PDFs need nothing beyond the app. OCR and legacy Office formats
rely on local tools; when a tool is missing the app says exactly what it looked
for instead of writing an empty or fake Markdown file.

## Build and run

```sh
cargo test          # library tests
cargo tauri dev     # run the desktop app
cargo tauri build --bundles dmg   # macOS installer
```

The queue logic is covered by tests: recursive folder scanning, deduplication,
deterministic ordering, collision-safe output names, atomic writes, and
cancellation. External tools are always invoked with explicit arguments and are
never passed through a shell.

## Layout

```
src/                 Rust extraction core and format adapters
src-tauri/           Tauri shell, commands, and packaging
web/                 UI (drop zone, queue, destination picker, results)
v1-swift/            Archived Mark-It-Down v1 SwiftUI source (reference only)
v1-artifacts/        v1 Windows build and client fixtures (local, gitignored)
docs/plans/          Product plan and evidence spec
docs/benchmarks/     Corpus inventory and pilot results
docs/history/        v1 design specs, hybrid PDF baseline, earlier plans
```

`v1-swift/` is the macOS predecessor, preserved because it holds the PDF
quality core this project has not yet fully ported: hybrid routing, per-page
OCR classification, and a measured ~10.4x speedup on native-text PDFs. See
`v1-swift/README.md`.

## Relationship to Mark-It-Down v1

v1 was consolidated into this repository on 2026-10-04. Its source, build
assets, documentation, and Windows portable bundle now live here under
`v1-swift/` and `v1-artifacts/`. The original repository remains available at
<https://github.com/arbiger/mark-it-down> with its full history, and is not
deprecated.

## Current limitations

- macOS build is Apple Silicon; Intel and Windows builds are not yet produced.
- OCR is not bundled — it requires a local Tesseract install, plus `pdftoppm`
  or `mutool` for scanned PDFs.
- Office extraction is text-oriented. Tables, formulas, and visual layout are
  not preserved the way v1's PDF path preserves reading order.
- PDFs needing OCR are not yet refused up front the way v1 refuses them; they
  fall through to the OCR path when a renderer is available.

## Records

- `DEV-LOG.md` — dated decisions and verification evidence
- `HANDOFF.md` — current state and what the next operator needs

## License

MIT — see [`LICENSE`](LICENSE). Third-party component terms are recorded in
[`THIRD_PARTY_NOTICES.md`](THIRD_PARTY_NOTICES.md); all direct dependencies are
permissive (MIT or MIT/Apache-2.0). Optional external tools such as Tesseract,
Poppler, MuPDF, and LibreOffice are detected and invoked locally, never
bundled, so their copyleft terms do not extend to this project.
