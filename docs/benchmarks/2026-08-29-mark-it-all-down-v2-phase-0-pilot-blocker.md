# Mark-It-All-Down v2 Phase 0 Pilot Status

**Status:** Draft — blocked before engine/OCR execution  
**Run ID:** `phase0-pilot-2026-08-29-blocked`  
**Date:** 2026-08-29 (Asia/Taipei)  
**Purpose:** Record why a meaningful Docling/Xberg or OCR comparison was not run.

## Read-only inspection

The local project contains one manual fixture under `Tests/ManualFixtures/`:

| Fixture ID | Visibility | Type/scenario | Size | SHA-256 | Use |
|---|---|---|---:|---|---|
| `manual-outlined-pdf-001` | `private-manual` | 14-page outlined/image-based PDF | 75,695,475 bytes | `c974cb66d4f70491742dc22e2a7d027102f810866447e66004621d4f10fb974e` | Metadata only; not copied or exposed |

The existing v1 benchmark documents four additional native-text PDFs and this
manual PDF, but those source files are not present in the project tree. They
cannot be rerun from documentation alone. No committed multi-format corpus,
reviewed gold assertions, English/Traditional Chinese paired fixtures, or
portable OCR test images were found.

## Environment evidence

| Check | Result |
|---|---|
| `docling` executable | Not found on PATH |
| `xberg` executable | Not found on PATH |
| Apple Vision runtime | Not runnable from this shell-only inspection |
| Portable OCR executable | `/opt/homebrew/bin/tesseract` exists; language/model coverage and approved candidate status not established |
| Network/download/install activity | None |
| v1 runtime/source/package changes | None |

## Decision and blocker

Do not run a comparison on the single private outlined PDF. It cannot establish
cross-format quality, language coverage, native-text behavior, or Mac/Windows
parity, and no gold truth is defined for it. The documented v1 baseline is
historical evidence only, not a Docling-vs-Xberg or OCR comparison.

To unblock the pilot, a human must approve a small frozen, redistributable
corpus (including native, scanned/mixed/outlined, English, and Traditional
Chinese cases), gold assertions, candidate installation sources/versions, and
the macOS/Windows test cells. After that approval, an isolated pilot may record
measured outputs using the protocol in the Phase 0 evidence specification.

## Explicit not-run fields

- Docling conversion: `not-run` — unavailable and no approved environment.
- Xberg conversion: `not-run` — unavailable and no approved environment.
- Apple Vision comparison: `not-run` — no approved corpus/gold set; shell-only inspection.
- Portable OCR comparison: `not-run` — no approved candidate/configuration or gold set.
- Windows execution/parity: `not-run` — no Windows environment available here.
- Timing, memory, output quality, packaging, license, and model measurements:
  `not-run` — pilot preconditions are unmet.

No engine, OCR provider, architecture, production readiness, or cross-platform
parity conclusion is made by this report.
