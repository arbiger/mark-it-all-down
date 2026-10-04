# Mark-It-All-Down v2 Product and Delivery Plan

**Date:** 2026-08-29  
**Status:** Draft for human approval  
**Product name:** Mark-It-All-Down  
**Relationship to v1:** A successor product that preserves the proven Mark-It-Down interaction model. The current v1 application remains intact.  
**Current authorization:** Product planning and specification only. No v2 repository, application code, dependency installation, packaging, publishing, or release is authorized by this document.

## 1. Decision summary

The proposed product is a local-first macOS and Windows desktop application that converts common documents, spreadsheets, presentations, publishing formats, PDFs, and document images into trustworthy Markdown.

The product will:

- preserve the current Mark-It-Down flow: drop files or folders, review, start, stop, and reveal results;
- accept one file, multiple files, one or more folders, or a mixture of files and folders;
- let the user write outputs beside each source or select a separate output folder;
- extract encoded text before considering OCR;
- apply OCR only to pages or images that need it;
- use format-specific conversion rules rather than flattening every source into plain text;
- keep original files unchanged and write outputs atomically;
- fail visibly on incomplete, empty, or unsupported conversions;
- process locally by default, with any future cloud or LLM enhancement explicitly enabled by the user.

The implementation foundation is intentionally not selected yet. Docling and Xberg will be evaluated against the same corpus before the architecture gate. OCRmyPDF is a PDF-repair/export component, not the main Markdown engine.

## 2. Task frame

### Goal

Define and deliver Mark-It-All-Down v2 as a dependable cross-platform application that turns most everyday text-bearing files and files requiring OCR into useful Markdown with preserved structure, assets, warnings, and source traceability.

### Scope

- macOS and Windows desktop applications;
- local document conversion and OCR;
- files and recursively scanned folders;
- same-folder and chosen-folder output modes;
- deterministic, collision-safe naming;
- conversion progress, cancellation, per-file results, and reveal actions;
- format-aware Markdown and companion assets;
- measurable engine selection and cross-platform acceptance.

### Non-goals

- editing or visually recreating source documents;
- pixel-perfect Markdown reproduction;
- executing Office macros, scripts, or embedded active content;
- using generative OCR or a cloud service by default;
- silently accepting partial or empty output;
- replacing specialist archival, records-management, or PDF-authoring software;
- adding v2 code to the existing SwiftUI v1 application before the architecture gate;
- supporting every historical or proprietary format in the first release.

### Success criteria

- the supported-format corpus converts without modifying any source hash;
- no empty or whitespace-only Markdown is reported as success;
- exact file selections remain exact, and only explicitly selected folders recurse;
- same-folder and chosen-folder output modes are deterministic and collision-safe;
- encoded text takes the fast path and never invokes OCR unnecessarily;
- scanned and mixed-content fixtures identify and OCR the correct pages or image regions;
- headings, lists, links, tables, slide order, sheet boundaries, and image references are preserved where the source exposes them;
- low-confidence or incomplete output is labeled for review rather than presented as complete;
- cancellation terminates active work and leaves no temporary outputs;
- an ordinary installed build can perform routine conversions offline after setup;
- the same acceptance corpus passes on supported macOS and Windows versions.

### Verification

- gold-file corpus comparison;
- source/output hash and path checks;
- automated unit, contract, integration, and cancellation tests;
- Markdown parsing and link/asset validation;
- OCR accuracy and page-order measurements;
- cold-start, warm-start, throughput, memory, and installer-size measurements;
- packaged application tests on clean macOS and Windows accounts;
- bounded GUI acceptance for drag/drop, file/folder picker, output selection, progress, Stop, and Reveal.

### Human gates

Human approval is required before:

- creating the v2 repository or adopting the proposed architecture;
- selecting Docling, Xberg, OCR engines, model weights, or commercial dependencies;
- bundling LibreOffice, Python, OCR models, or other large runtimes;
- enabling network access, telemetry, cloud OCR, or LLM processing;
- signing, notarizing, publishing, or distributing an application;
- changing the current Mark-It-Down v1 application or release.

## 3. Product promise

> Drop files or folders, choose where the results go, and receive trustworthy Markdown without having to know which extraction or OCR engine is required.

“Most files” means a documented support matrix with quality levels. It does not mean that every extension is treated identically or that every file can be represented completely in Markdown.

## 4. Format support target

### 4.1 Release-one core formats

| Category | Extensions | Required behavior |
|---|---|---|
| PDF | `.pdf` | Native extraction, page-aware OCR when required, reading order, tables, images, and explicit incomplete-page warnings |
| Images | `.png`, `.jpg`, `.jpeg`, `.tif`, `.tiff`, `.bmp`, `.webp` | OCR for document-like images; retain original asset; do not invent a caption for ordinary photographs |
| Word processing | `.docx`, `.odt`, `.rtf` | Headings, paragraphs, lists, links, footnotes, tables, and embedded images |
| Spreadsheets | `.xlsx`, `.ods`, `.csv`, `.tsv` | Workbook and sheet boundaries, displayed values, formulas where available, tables, and large-sheet companion files |
| Presentations | `.pptx`, `.odp` | Slide order, title/body text, speaker notes, tables, and embedded images |
| Publishing/web | `.html`, `.xhtml`, `.epub` | Document/chapter hierarchy, links, lists, tables, and embedded assets |
| Text/markup | `.txt`, `.md`, `.rst` | Encoding-safe text and structure preservation without unnecessary rewriting |

### 4.2 Release-one legacy compatibility target

| Category | Extensions | Required behavior |
|---|---|---|
| Legacy Word | `.doc`, `.dot` | Extract without executing macros; use a compatibility adapter or isolated normalization fallback |
| Legacy Excel | `.xls`, `.xlt` | Preserve sheet boundaries and displayed cell data; formulas when the parser exposes them |
| Legacy PowerPoint | `.ppt`, `.pot` | Preserve slide order, visible text, and notes when available |
| Macro-enabled Office | `.docm`, `.xlsm`, `.pptm` | Extract passive content only; never execute macros or embedded scripts |

### 4.3 OpenDocument clarification

ODF is a family of formats:

- `.odt`: text document;
- `.ods`: spreadsheet;
- `.odp`: presentation;
- `.odg`: drawing;
- `.odf`: mathematical formula.

The first three are release-one targets. `.odg` and `.odf` are extended targets; formulas should become LaTeX when reliable, while drawings should retain the source asset and extract embedded text.

### 4.4 Extended formats after release one

- Apple iWork: `.pages`, `.numbers`, `.key`;
- email: `.eml`, `.msg`;
- vector and drawing formats: `.svg`, `.odg`;
- scientific and technical: `.tex`, `.ipynb`, structured `.xml` families;
- structured data: `.json`, `.yaml`, `.toml`;
- archives: `.zip`, `.7z`, `.tar`, subject to nesting and expansion limits;
- audio/video transcription only as a separately approved capability.

Extended formats must not weaken release-one reliability or force large default dependencies into the installer.

## 5. UX contract

### 5.1 Main workflow

1. Drop one or more files, one or more folders, or a mixture onto the window; alternatively use **Choose Files** or **Choose Folder**.
2. The application expands only explicitly selected folders, deduplicates overlapping selections, and shows the exact conversion queue.
3. Each row shows the source name, type, support status, planned route, and eventual result.
4. Select the output mode:
   - **Same folder as source** (default);
   - **Choose output folder…**.
5. Select **Convert to Markdown**.
6. Observe file-level and, when relevant, page-level progress.
7. Use **Stop** to cancel active and queued work.
8. Review succeeded, succeeded-with-warning, failed, cancelled, and unsupported results.
9. Use **Reveal** for an individual output or **Reveal Results** for the destination.

### 5.2 Main-screen design principle

Keep the current Mark-It-Down UI simple. OCR language, searchable-PDF export, advanced table behavior, structured manifests, and future cloud enhancement belong in a collapsed **Conversion Options** surface, not the default workflow.

### 5.3 Folder scanning rules

- recurse only folders explicitly selected by the user;
- preserve explicitly selected individual files;
- deduplicate a file selected directly and also discovered under a selected folder;
- ignore hidden files, temporary Office files such as `~$file.docx`, generated asset folders, and the active output tree;
- identify symlink loops and archive-expansion hazards;
- sort the queue deterministically;
- show unsupported files without pretending they converted.

## 6. Output contract

### 6.1 Same-folder mode

```text
Documents/
├── report.pdf
├── report.md
├── manual.docx
├── manual.md
├── sales.xlsx
├── sales.md
└── sales.assets/
    ├── Sheet1.csv
    └── Sheet2.csv
```

### 6.2 Chosen-folder mode

Preserve relative source structure:

```text
Source/
├── Reports/report.pdf
└── Manuals/manual.docx

Chosen Output/
├── Reports/report.md
└── Manuals/manual.md
```

When inputs come from unrelated roots, create one deterministic top-level group per selected root instead of flattening all files into one directory.

### 6.3 Naming and collision rules

- default output: `<source-stem>.md`;
- never overwrite an existing source or output;
- if two different source extensions collide, prefer `<source-name-with-extension>.md`, such as `report.pdf.md` and `report.docx.md`;
- if a collision remains, add a deterministic numeric suffix;
- write to a temporary path and atomically promote only a complete result;
- a cancelled, failed, empty, or structurally invalid result must not leave a final Markdown file.

### 6.4 Companion outputs

- simple sources produce one Markdown file;
- extracted images and oversized sheet data use `<stem>.assets/`;
- large sheets use CSV or JSON companion files rather than enormous Markdown tables;
- a structured provenance manifest is available as an option and may later support RAG workflows;
- the default output remains uncluttered for ordinary users.

### 6.5 Format-specific Markdown rules

#### Documents

Preserve semantic headings, paragraphs, lists, links, footnotes, tables, code, and image references. Page layout, fonts, tracked changes, and comments that cannot be represented faithfully belong in warnings or the optional manifest.

#### Spreadsheets

Create a workbook index, one section per sheet, and readable tables for bounded ranges. Preserve formulas and displayed values when available. Emit very large sheets as companion CSV/JSON files with Markdown summaries and links.

#### Presentations

Create one section per slide in source order. Preserve slide title, body, tables, speaker notes, and image references. OCR meaningful text inside embedded images without mixing it silently into the authored slide text.

#### Images

Classify an image as document-like or general imagery. OCR document-like images. For ordinary images, retain the source reference and metadata; do not generate descriptions unless the user explicitly enables a vision feature.

#### PDFs

Keep page provenance. Extract encoded text first, selectively OCR scanned or outlined pages, merge in page order, and surface unreadable or low-confidence pages. Do not treat a partially recovered document as complete without a warning.

## 7. Proposed architecture

### 7.1 Architectural principle

Mark-It-All-Down owns the routing, safety, output, and quality contract. No third-party converter owns the product architecture.

```text
Desktop UI
    ↓
Intake and exact-selection scanner
    ↓
Format detector and conversion planner
    ↓
Specialized adapters
    ├── native structured extraction
    ├── PDF extraction and page classification
    ├── document-image OCR
    ├── Office/OpenDocument conversion
    ├── advanced layout fallback
    └── optional searchable-PDF repair
    ↓
App-owned canonical document model
    ↓
Quality and completeness gate
    ↓
Markdown renderer + assets + optional manifest
    ↓
Atomic output writer
```

### 7.2 Provisional application shape

- cross-platform desktop shell: Tauri with a Rust orchestration layer is the leading option;
- conversion backends: isolated adapters or sidecars with pinned versions and explicit contracts;
- canonical model: app-owned document/page/block/table/asset/provenance structures;
- UI: reuse the current interaction and visual simplicity, not the macOS-only SwiftUI implementation;
- networking: disabled for ordinary conversion unless a user enables a separately approved feature.

This is a recommendation, not an adopted architecture. Packaging, accessibility, updater behavior, signed-child-process behavior, installer size, and Windows antivirus friction must be demonstrated first.

## 8. Engine evaluation

### 8.1 Foundation candidates

#### Docling

Strengths:

- broad modern Office, OpenDocument, PDF, image, HTML, EPUB, email, and publishing support;
- unified document model and Markdown/JSON/chunk exports;
- selectable OCR engines including macOS Vision, RapidOCR, and Tesseract;
- strong layout, table, reading-order, and provenance orientation.

Risks:

- Python/model/runtime footprint;
- legacy DOC/XLS/PPT relies on LibreOffice;
- model startup and packaging cost must be measured;
- output quality and speed can vary by pipeline configuration.

Reference: <https://github.com/docling-project/docling>

#### Xberg

Strengths:

- Rust core and broad native bindings;
- modern and legacy Office, OpenDocument, PDF, image, email, archive, and structured-data coverage;
- Markdown rendering, OCR backends, layout/table options, timeouts, and provenance;
- potential for a smaller, faster, more native cross-platform integration.

Risks:

- newer product/rebrand with less project-specific evidence;
- broad support claims need confirmation on real files;
- complex PDFs and Markdown fidelity must be measured against Docling;
- optional model and feature packaging still requires careful license/runtime review.

Reference: <https://github.com/xberg-io/xberg>

### 8.2 OCR and specialized components

| Component | Proposed role | Boundary |
|---|---|---|
| Apple Vision | Fast official macOS OCR provider | Mac-only; use public APIs only |
| RapidOCR/ONNX | Portable local OCR baseline | Benchmark CJK accuracy and packaging |
| PaddleOCR | Cross-platform OCR/layout candidate | Heavier runtime; benchmark before bundling |
| OCRmyPDF | Optional PDF cleanup and searchable-PDF export | Not a Markdown engine |
| Marker | Advanced layout/PDF fallback | Model/runtime and model-weight license review required |
| MinerU | Complex scientific-document fallback candidate | Heavy; not a default desktop path |
| Microsoft MarkItDown | Compatibility/reference adapter | Do not make it the v2 architecture owner |

Generative OCR is not the default. It may be evaluated later only with hallucination checks, page/region provenance, privacy controls, and explicit user choice.

### 8.3 Architecture bake-off gate

Run Docling and Xberg against the same frozen corpus on macOS and Windows. Measure:

- supported-format success rate;
- normalized content coverage;
- headings, lists, table, sheet, slide, and reading-order fidelity;
- OCR quality for English and Traditional Chinese;
- page and asset provenance;
- cold and warm conversion time;
- peak memory and sustained batch behavior;
- runtime, model, and installer size;
- cancellation, timeout, malformed-file, and password-protected-file behavior;
- licensing and redistributability;
- ease of producing deterministic signed macOS and Windows packages.

The gate may select one foundation or a measured router using both. It must not select an engine solely from repository popularity or self-reported benchmarks.

## 9. Reliability, privacy, and security requirements

- hash or otherwise verify that source files remain unchanged in acceptance tests;
- never execute Office macros, document scripts, embedded binaries, or files discovered inside archives;
- enforce input-size, page-count, decompression-ratio, nesting-depth, time, and memory limits;
- treat passwords as ephemeral user input and never record them in logs;
- sanitize asset filenames and prevent path traversal;
- keep subprocess arguments explicit and avoid shell interpretation;
- isolate temporary work and remove it on success, failure, and cancellation;
- keep logs under the platform-appropriate application log directory and exclude document content by default;
- make network use visible and off by default;
- pin conversion dependencies and record versions in diagnostics;
- complete a transitive license and model-weight review before distribution.

## 10. Delivery phases

### Phase 0 — Product and evidence gate

Deliverables:

- approved product target and support matrix;
- frozen representative corpus with redistributable automated fixtures and private/manual fixtures separated;
- gold-output or gold-assertion definitions;
- Docling/Xberg benchmark report;
- OCR provider comparison on macOS and Windows;
- architecture decision record;
- dependency, model, license, installer-size, and privacy assessment.

Exit gate: human approves the architecture and the first implementation slice.

### Phase 1 — Cross-platform shell and safe intake

Deliverables:

- new v2 repository;
- macOS and Windows development builds;
- drag/drop and picker intake for exact files and recursive folders;
- queue preview, deduplication, supported-format status, and deterministic ordering;
- same-folder and chosen-folder destinations;
- collision-safe atomic output writer;
- Stop, per-file results, and Reveal;
- minimal text/HTML/modern Office adapters without OCR.

Exit gate: path, collision, cancellation, source-integrity, and clean-machine smoke tests pass on both platforms.

### Phase 2 — Core document conversion

Deliverables:

- PDF native-text path;
- DOCX/ODT/RTF;
- XLSX/ODS/CSV/TSV with sheet-aware output;
- PPTX/ODP with slide-aware output;
- EPUB and image/asset extraction;
- canonical model and Markdown renderer;
- warnings and optional provenance manifest.

Exit gate: release-one digital-document corpus meets the approved content and structure thresholds.

### Phase 3 — Local OCR and mixed documents

Deliverables:

- macOS OCR provider using official Apple APIs or the approved shared provider;
- Windows OCR provider;
- document-image classification;
- page-selective OCR for scanned, mixed, and outlined PDFs;
- embedded-image OCR for Office and OpenDocument formats;
- page/region confidence, provenance, and review warnings;
- Traditional Chinese and English acceptance fixtures.

Exit gate: OCR accuracy, routing, cancellation, memory, and cross-platform consistency thresholds pass.

### Phase 4 — Legacy and difficult formats

Deliverables:

- DOC/XLS/PPT compatibility adapters;
- macro-safe handling;
- optional isolated LibreOffice normalization only if required;
- advanced layout fallback for difficult tables, formulas, and multi-column documents;
- password-protected and malformed-file UX.

Exit gate: legacy corpus and security tests pass; installer impact is approved.

### Phase 5 — Packaging and release candidate

Deliverables:

- reproducible macOS and Windows installers;
- dependency hashes and third-party notices;
- signed binaries and child processes;
- clean-account first-run and offline-repeat tests;
- accessibility and high-volume batch acceptance;
- upgrade, uninstall, log, and cache behavior;
- release notes and user documentation.

Exit gate: human approves signing, notarization, publishing, and distribution. Local tests alone do not establish production readiness.

## 11. Test corpus design

The corpus should include small redistributable automated fixtures plus private/manual real-world samples.

Minimum categories:

- clean and malformed native-text PDFs;
- scanned PDFs, mixed PDFs, outlined text, rotated pages, low-resolution pages, and multi-column reading order;
- English, Traditional Chinese, Simplified Chinese, and mixed-language documents;
- DOCX/ODT with headings, lists, tables, footnotes, hyperlinks, tracked changes, comments, and embedded images;
- XLSX/XLS/ODS with multiple sheets, formulas, merged cells, hidden rows, sparse ranges, charts, and very large sheets;
- PPTX/PPT/ODP with speaker notes, grouped objects, tables, diagrams, and text inside images;
- document images, photographs without meaningful text, screenshots, multi-page TIFF, handwriting, and skew;
- HTML/EPUB with nested navigation, links, tables, images, and unusual encodings;
- password-protected, corrupt, oversized, unsupported, deceptive-extension, and archive-bomb fixtures;
- duplicate names, cross-root selections, symlink loops, output-inside-input, cancellation, and disk-full simulations.

Gold checks should test important facts instead of relying only on whole-file text equality. Examples: required heading hierarchy, expected sheet count, expected slide order, table dimensions, page coverage, link targets, asset presence, and prohibited invented content.

## 12. Acceptance checklist

### Intake and output

- [ ] Exact file selection does not pull neighboring files.
- [ ] Only selected folders recurse.
- [ ] Overlapping selections deduplicate deterministically.
- [ ] Same-folder output writes beside each source.
- [ ] Chosen-folder output preserves relative structure.
- [ ] Cross-root inputs receive deterministic root grouping.
- [ ] Existing outputs are never overwritten.
- [ ] Output-inside-input does not recurse into generated results.
- [ ] Original hashes remain unchanged.

### Content and OCR

- [ ] Digital text never invokes OCR without a documented reason.
- [ ] Scanned, mixed, and outlined pages route correctly.
- [ ] Empty or whitespace-only output fails.
- [ ] Partial and low-confidence results carry warnings.
- [ ] Pages, sheets, and slides remain in source order.
- [ ] Tables and assets are present or explicitly reported as unsupported.
- [ ] OCR does not fabricate descriptions for ordinary images.
- [ ] English and Traditional Chinese gold checks meet the thresholds approved at Phase 0.

### Runtime and safety

- [ ] Stop terminates work promptly and cleans temporary files.
- [ ] Per-file timeout and size limits work.
- [ ] Macros and active content never execute.
- [ ] Routine conversion works offline after setup.
- [ ] Logs contain routes, versions, durations, and errors without source content by default.
- [ ] Packaged builds pass on clean macOS and Windows accounts.
- [ ] Dependency licenses, hashes, and model terms are reviewed before distribution.

## 13. Red/blue review prompts

### Blue review

- Does the product keep the current UI simple?
- Are file-format differences represented honestly?
- Are outputs useful for people as well as downstream AI/RAG workflows?
- Are exact selection, collision safety, cancellation, and reveal behavior retained?
- Is the plan implementable in bounded, verifiable slices?

### Red review

- Does “most files” conceal weak or partial support?
- Could OCR be invoked unnecessarily or silently hallucinate content?
- Could chosen-folder routing flatten paths, overwrite files, or recurse into itself?
- Could a legacy Office file execute active content?
- Could a converter, model, or license prevent commercial or public distribution?
- Could one foundation engine become an unreplaceable architecture dependency?
- Could Mac output differ materially from Windows while both are labeled successful?
- Could very large spreadsheets, PDFs, images, or archives exhaust memory or disk?
- What evidence would disprove each success claim?

## 14. Decisions still requiring human approval

1. Approve this product target and release-one support matrix.
2. Approve Phase 0 benchmarking before creating application code.
3. Decide whether identical cross-platform OCR behavior is more important than using the best native engine on each platform.
4. Decide whether optional structured manifests should be visible by default or remain an advanced output.
5. Approve the architecture only after Docling/Xberg and OCR evidence is available.
6. Approve the creation and location of the new Mark-It-All-Down repository.
7. Approve packaging, signing, publishing, and any network/cloud capability separately.

## 15. Recommended next action

After human approval of this plan, perform Phase 0 only:

1. create an exact corpus inventory and gold-check schema;
2. run a read-only Docling/Xberg conversion bake-off on macOS;
3. define the corresponding Windows test environment;
4. compare Apple Vision with the approved portable OCR candidates;
5. record the architecture decision and installer/license consequences;
6. return for human approval before creating the v2 repository or implementing the UI.
