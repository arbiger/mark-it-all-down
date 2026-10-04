# Mark-It-All-Down v2 Phase 0 Evidence and Benchmark Specification

**Status:** Draft — Phase 0 preparation; not an implementation authorization  
**Date:** 2026-08-29  
**Parent plan:** `2026-08-29-mark-it-all-down-v2-product-plan.md`

## Purpose and boundary

This specification defines reproducible evidence needed before selecting a v2
conversion foundation or creating a v2 repository. It does not select Docling,
Xberg, an OCR provider, a model, a packaging strategy, or an architecture.
Existing v1 files, source code, fixtures, and behavior are read-only scope.

The repository currently has no committed v2 corpus. The existing hybrid-PDF
benchmark is a useful macOS metadata/timing baseline, but is not a substitute
for the frozen multi-format corpus or semantic gold checks defined here.

## Evidence rules

Every result MUST identify: run ID, operator, timestamp and timezone, host,
OS/build, tool and dependency versions, configuration, input fixture ID and
SHA-256, output SHA-256/size, elapsed time, exit status, warnings, and whether
the result is measured, manually reviewed, or an inference. Keep raw logs and
private source content outside committed artifacts; commit only approved,
redistributable fixtures and aggregate evidence. Never alter an input fixture.

Record claims separately from evidence:

| Claim | Required evidence | Status vocabulary |
|---|---|---|
| Text/structure is preserved | Gold checks and reviewer record | `pass`, `fail`, `partial`, `not-applicable` |
| OCR is accurate | Character/word/region metrics plus language and fixture IDs | `measured`, `not-measured` |
| Output is complete | Page/sheet/slide coverage and warning checks | `pass`, `fail`, `warning` |
| Route is safe | Route trace, source hash, cancellation/temp-file checks | `pass`, `fail`, `not-run` |
| Runtime is suitable | Repeated cold/warm measurements and resource samples | measured values only |

Do not compare scores across different corpus versions, preprocessing, or
hardware without recording the difference. A failed or incomplete conversion
must not be counted as a successful output merely because a process exited 0.

## Corpus inventory schema

Use `docs/benchmarks/mark-it-all-down-v2-corpus-inventory.csv` as the row
template. One row represents one immutable fixture or one private/manual sample.

| Field | Required meaning |
|---|---|
| `fixture_id` | Stable ID, never reused; no personal filename required |
| `corpus_version` | Frozen corpus release identifier |
| `visibility` | `redistributable`, `private-manual`, or `synthetic` |
| `category` / `format` / `extension` | Plan support category and actual format |
| `language` | `en`, `zh-Hant`, `zh-Hans`, `mixed`, or other declared value |
| `scenario` | e.g. native-text, scanned, mixed, outlined, malformed, safety |
| `source_label` | Human-readable non-sensitive label |
| `relative_path` | Path inside the controlled corpus, never an absolute private path |
| `sha256` / `bytes` | Input integrity and size |
| `pages_or_units` | Pages, sheets, slides, chapters, or image count as applicable |
| `expected_route` | Native, selective-OCR, image-OCR, unsupported, or failure-test |
| `gold_ref` | Gold assertion file/ID; do not embed private content |
| `license_ref` | Source license/permission evidence |
| `notes` | Known layout, language, privacy, or review caveat |

Minimum coverage follows the parent plan: native/scanned/mixed/outlined PDFs;
English and Traditional Chinese; documents, spreadsheets, presentations,
images, HTML/EPUB, text/markup; malformed/password/oversized/unsupported
inputs; and intake/collision/cancellation safety scenarios. Separate private
real-world material from small redistributable automated fixtures.

## Gold-check schema

Gold checks are assertions, not necessarily whole-output byte equality. Store
one record per assertion with these fields:

| Field | Meaning |
|---|---|
| `check_id`, `fixture_id`, `dimension` | Stable identity; `content`, `structure`, `ocr`, or `provenance` |
| `scope` | Document, page, sheet, slide, region, block, table, link, or asset |
| `assertion_type` | Exact text, normalized text, heading tree, order, count, table shape, link target, asset reference, OCR metric, route, or warning |
| `expected` | Controlled expected value or numeric target; never an unapproved threshold |
| `tolerance` | Explicit normalization/numeric tolerance, otherwise `exact` |
| `actual` / `status` | Measured value and `pass`/`fail`/`partial`/`not-run` |
| `evidence_ref` | Output/log/reviewer record reference |
| `reviewer`, `reviewed_at` | Required for manual semantic checks |

Content checks cover required text, no invented text, encoding, and displayed
values. Structure checks cover headings, lists, links, table dimensions,
page/sheet/slide order, boundaries, and assets. OCR checks cover language,
CER/WER where a transcript exists, region/text detection, confidence
distribution, page coverage, and false additions. Provenance checks cover
source hash, page/unit/region origin, route reason, warnings, output path, and
atomic/no-partial-output behavior. Human approval is required for release
thresholds and normalization rules.

## Docling vs Xberg bake-off protocol

1. Freeze the corpus version, input hashes, language set, and test configurations.
2. Run each candidate in isolated, documented environments on the same host
   class. Record exact versions, model assets, environment variables, and
   preprocessing; do not download dependencies or models as part of this
   preparation artifact.
3. Use equivalent settings: local/offline mode, no network, no generative
   enhancement, and the same output destination policy. Run at least three
   cold processes and three warm conversions per fixture when execution is
   authorized; randomize engine order.
4. Capture exit status, route, output presence, Markdown parse validity,
   warnings, page/unit coverage, gold-check results, elapsed time, peak memory,
   output size, and temporary-file cleanup.
5. Report medians and spread for timing/resource data. Report per-category
   quality; never use a single aggregate score to hide a failed category.
6. Repeat the approved subset on Windows with the same corpus hashes and
   equivalent settings. Explain unavoidable platform differences.

The bake-off may produce an evidence-backed recommendation only. It cannot
adopt an engine until the human architecture gate below is approved.

## Apple Vision vs portable OCR protocol

Compare Apple Vision on macOS against each portable candidate approved for
testing (for example, RapidOCR/ONNX or another explicitly approved option),
and compare the Windows implementation of the same portable candidate. Use
identical rendered page/image inputs, DPI, crop/deskew policy, language hints,
and post-processing wherever technically possible. Record any non-equivalence.

Measure English, Traditional Chinese, mixed-language, rotated, low-resolution,
multi-column, handwriting (if in scope), and ordinary-photo negative controls.
Report CER/WER against reviewed transcripts, region/page recall, false text,
ordering, confidence calibration, latency, peak memory, offline behavior, and
model/runtime/installer footprint. Verify that ordinary photos do not receive
invented descriptions and that low-confidence output is flagged. Native OCR
quality must not be inferred from vendor claims or a different corpus.

## Environment matrix

| Axis | macOS test cell | Windows test cell | Evidence to record |
|---|---|---|---|
| Supported release | Exact approved macOS version + Apple Silicon/Intel status | Exact approved Windows 10/11 version + x64/ARM status | OS build and architecture |
| Clean machine | New user/account, no developer tools assumed | New user/account, no developer tools assumed | Setup transcript and offline rerun |
| Development | Pinned toolchain and runtime, explicitly documented | Pinned toolchain and runtime, explicitly documented | Versions and hashes |
| OCR | Apple Vision plus approved portable candidate | Approved portable candidate and Windows-native comparison if authorized | Language packs/models and terms |
| Packaging | Unsigned test bundle first; signing is later human-gated | Installer/test bundle first; signing is later human-gated | Size breakdown and launch log |
| Resource cells | Cold/warm, low-memory, large-file, cancellation | Same scenarios and equivalent limits | Time, memory, disk, cleanup |

No parity claim is valid if a cell is skipped; label it `not-run` and explain.

## License, model, dependency, installer, privacy checklist

For every direct and transitive dependency, runtime, model, language pack, and
bundled binary, record name/version/source URL, immutable hash, license,
copyright/notice obligations, redistribution and commercial-use terms, source
offer obligations, platform restrictions, and whether network access is
required. Review model cards and weight licenses separately from code licenses.

Measure uncompressed and compressed installer size, per-component size,
first-run cache/model download behavior, disk footprint, startup time, memory,
antivirus/notarization implications, and offline repeat conversion. Confirm
that no private fixture, document content, password, or credential enters
logs/telemetry. Network, telemetry, cloud OCR, and LLM features remain off
unless separately approved. Do not bundle a component solely because it is
convenient; record its measured necessity and an alternative.

## Stop conditions and human approval gate

Stop and return for a human decision when: corpus permission or model terms are
unclear; a candidate requires unapproved network/cloud behavior; a license,
installer impact, or platform restriction is unresolved; a fixture is missing
or its gold truth is disputed; Mac/Windows results are not comparable; output
is partial, empty, reordered, fabricated, or untraceable; source hashes change;
temporary files survive cancellation; or a result would require changing v1.

Phase 0 ends only when the corpus, gold assertions, bake-off reports, OCR
comparison, environment results, and dependency/privacy assessment are
complete enough for review. A human must explicitly approve the architecture,
engine/OCR choices, model/dependency packaging, and first implementation slice
before any v2 repository, code, install, package, signing, or publication work.

### Deferred/not executed in this preparation

No conversion benchmark, dependency installation, model download, network
download, package build, repository creation, or v1 modification was performed.
