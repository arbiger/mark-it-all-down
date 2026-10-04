# Third-party notices

Mark-It-All-Down is MIT licensed (see `LICENSE`). That license covers this
project's own code only. The components below are developed and licensed by
others, and their terms apply to them independently.

All direct dependencies were verified permissive (MIT or MIT/Apache-2.0) as of
2026-10-04. Being permissive, they do not constrain this project's own license
— they require only that their notices be preserved, which this file does.

## Rust dependencies (shipped in the app)

| Crate | Version | License | Project |
|---|---|---|---|
| `pdf-extract` | 0.7 | MIT | <https://crates.io/crates/pdf-extract> |
| `quick-xml` | 0.37 | MIT | <https://crates.io/crates/quick-xml> |
| `zip` | 2.2 | MIT | <https://crates.io/crates/zip> |
| `tauri` | 2.11 | MIT OR Apache-2.0 | <https://github.com/tauri-apps/tauri> |
| `tauri-build` | 2.6 | MIT OR Apache-2.0 | <https://github.com/tauri-apps/tauri> |
| `tauri-plugin-dialog` | 2.7 | MIT OR Apache-2.0 | <https://github.com/tauri-apps/plugins-workspace> |
| `tauri-plugin-log` | 2.9 | MIT OR Apache-2.0 | <https://github.com/tauri-apps/plugins-workspace> |
| `tauri-plugin-opener` | 2.5 | MIT OR Apache-2.0 | <https://github.com/tauri-apps/plugins-workspace> |
| `serde`, `serde_json`, `log` | — | MIT OR Apache-2.0 | <https://crates.io> |

Transitive dependencies retain their own license files in the Cargo registry
cache and are not enumerated here. Run `cargo license` or
`cargo about --all` for a full generated inventory before a public release.

## Python dependencies (v1 legacy runtime, retained under `v1-swift/`)

The archived Mark-It-Down v1 code under `v1-swift/` installs these into a
private per-user virtual environment. They are **not** bundled into the v2
application binary; they remain documented because the v1 source is preserved
for reference and can still be built.

### Microsoft MarkItDown 0.1.7

- Project: <https://github.com/microsoft/markitdown>
- Copyright: Microsoft Corporation and contributors
- License: MIT

### Firecrawl PDF Inspector 0.2.6

- Project: <https://github.com/firecrawl/pdf-inspector>
- Copyright (c) 2026 Firecrawl
- License: MIT

`v1-swift/requirements-macos.txt` is a direct-pin file, not a complete
hash-locked transitive inventory. Before redistributing anything that bundles
the v1 runtime, generate a full transitive license and hash inventory.

## Optional external tools (not redistributed)

Mark-It-All-Down detects and shells out to these local programs when present.
They are **not** bundled or redistributed by this project, and their licenses
are the user's responsibility to satisfy on their own machines:

- **Tesseract OCR** — Apache-2.0, <https://github.com/tesseract-ocr/tesseract>
- **Poppler** (`pdftotext`, `pdftoppm`) — GPL-2.0-or-later, <https://poppler.freedesktop.org>
- **mutool** (MuPDF) — AGPL-3.0-or-later, <https://mupdf.com>
- **LibreOffice** — MPL-2.0, <https://www.libreoffice.org>

Because these are invoked as separate local executables rather than linked or
redistributed, this project does not become subject to their copyleft terms.
Shipping any of them inside a future bundle would change that analysis and
requires a fresh license review.

## MIT License text

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
