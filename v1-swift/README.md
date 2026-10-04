# Mark-It-Down v1 (archived reference)

This directory preserves the complete SwiftUI source of **Mark-It-Down v1**, the
macOS-only predecessor to Mark-It-All-Down. It is kept for reference and is not
part of the v2 build.

## What v1 did that v2 does not yet

v1 is the PDF quality core that this project inherits from:

- **Hybrid PDF routing** (`Sources/MarkItDown/HybridConversion.swift`) — routes
  PDFs to Firecrawl PDF Inspector first and falls back to Microsoft MarkItDown
  for other formats.
- **Per-page OCR classification** — a PDF that needs OCR is reported page by
  page and produces **no Markdown file at all**, rather than a partial one.
- **Private Python runtime** (`Sources/MarkItDown/PythonLocator.swift`,
  `MarkitdownInstaller.swift`) — builds a per-user virtual environment pinned to
  `pdf-inspector==0.2.6` and `markitdown[all]==0.1.7`.
- **Measured speedup** — roughly 10.4x median on native-text PDF fixtures
  (see `docs/history/2026-08-04-hybrid-pdf-baseline.md`).
- **41 Swift tests**, including cancellation, cleanup, bootstrap repair, and
  JSON-contract coverage.

v1's current PDF extraction in v2 is intentionally simpler. Porting the
classification discipline and routing behaviour into the Rust core is the main
outstanding quality gap in this project.

## Licensing

`THIRD_PARTY_NOTICES.md` records the v1 dependency notices, including the
Firecrawl PDF Inspector and Microsoft MarkItDown terms. Read it before reusing
v1 code or vendoring those engines into a distributed build.

## Upstream

The live v1 repository is <https://github.com/arbiger/mark-it-down>. This copy
is a snapshot of the hybrid PDF engine work (commit `2729685`).
