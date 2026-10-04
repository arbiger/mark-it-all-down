use quick_xml::events::Event;
use quick_xml::Reader;
use std::{
    fs, io,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use zip::ZipArchive;

fn resolve_executable(names: &[&str]) -> Result<PathBuf, String> {
    let dirs = [
        "/opt/homebrew/bin",
        "/usr/local/bin",
        "/usr/bin",
        "/Applications/LibreOffice.app/Contents/MacOS",
    ];
    let mut searched = Vec::new();
    for name in names {
        if let Ok(path) = std::env::var("PATH") {
            for dir in path.split(':') {
                let p = PathBuf::from(dir).join(name);
                searched.push(p.display().to_string());
                if p.is_file() {
                    return Ok(p);
                }
            }
        }
        for dir in dirs {
            let p = PathBuf::from(dir).join(name);
            searched.push(p.display().to_string());
            if p.is_file() {
                return Ok(p);
            }
        }
    }
    Err(format!(
        "executable not found (searched: {})",
        searched.join(", ")
    ))
}

pub const SUPPORTED: &[&str] = &[
    "md", "txt", "rst", "html", "xhtml", "epub", "pdf", "png", "jpg", "jpeg", "tif", "tiff", "bmp",
    "webp", "docx", "odt", "rtf", "xlsx", "ods", "csv", "tsv", "pptx", "odp", "doc", "dot", "xls",
    "xlt", "ppt", "pot", "docm", "xlsm", "pptm",
];

pub fn expand_inputs(files: &[PathBuf], folders: &[PathBuf]) -> io::Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    for p in files {
        if p.is_file() {
            out.push(fs::canonicalize(p)?);
        }
    }
    for root in folders {
        walk(root, &mut out)?;
    }
    out.sort();
    out.dedup();
    Ok(out)
}
fn walk(root: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
    if !root.is_dir() {
        return Ok(());
    }
    for e in fs::read_dir(root)? {
        let p = e?.path();
        if p.file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.starts_with('.') || n.starts_with("~$"))
            .unwrap_or(false)
        {
            continue;
        }
        if p.is_dir() {
            walk(&p, out)?
        } else if p.is_file() {
            out.push(fs::canonicalize(p)?)
        }
    }
    Ok(())
}
pub fn supported(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| SUPPORTED.contains(&e.to_ascii_lowercase().as_str()))
        .unwrap_or(false)
}
pub fn output_path(
    source: &Path,
    roots: &[PathBuf],
    chosen: Option<&Path>,
    used: &mut Vec<PathBuf>,
) -> PathBuf {
    let base = chosen
        .map(PathBuf::from)
        .unwrap_or_else(|| source.parent().unwrap_or(Path::new(".")).to_path_buf());
    let rel = roots
        .iter()
        .find_map(|r| source.strip_prefix(r).ok())
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(source.file_name().unwrap_or_default()));
    let stem = rel.file_stem().unwrap_or_default().to_string_lossy();
    let parent = base.join(rel.parent().unwrap_or(Path::new("")));
    let _ = fs::create_dir_all(&parent);
    let mut candidate = parent.join(format!("{stem}.md"));
    let mut n = 1;
    while candidate == source || candidate.exists() || used.contains(&candidate) {
        candidate = parent.join(format!("{stem} ({n}).md"));
        n += 1;
    }
    used.push(candidate.clone());
    candidate
}
pub fn atomic_write(path: &Path, content: &str) -> io::Result<()> {
    let tmp = path.with_extension("md.tmp");
    fs::write(&tmp, content)?;
    fs::rename(tmp, path)
}
fn xml_text(xml: &str) -> String {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut out = String::new();
    loop {
        match reader.read_event() {
            Ok(Event::Text(t)) => {
                if let Ok(value) = t.unescape() {
                    let value = value.trim();
                    if !value.is_empty() {
                        out.push_str(value);
                        out.push('\n');
                    }
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(_) => break,
        }
    }
    out
}
pub fn stub(path: &Path, cancel: &Arc<AtomicBool>) -> Result<String, String> {
    if cancel.load(Ordering::Relaxed) {
        return Err("cancelled".into());
    }
    if !supported(path) {
        return Err("unsupported format; no conversion performed".into());
    }
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if ext == "pdf" {
        if let Ok(text) = pdf_extract::extract_text(path) {
            if !text.trim().is_empty() {
                return Ok(text);
            }
        }
        let pdftotext = resolve_executable(&["pdftotext"])?;
        let result = std::process::Command::new(pdftotext)
            .arg("-layout")
            .arg(path)
            .arg("-")
            .output()
            .map_err(|e| format!("PDF extractor unavailable: {e}"))?;
        if !result.status.success() {
            return Err(format!(
                "PDF text extraction failed: {}",
                String::from_utf8_lossy(&result.stderr).trim()
            ));
        }
        let text = String::from_utf8_lossy(&result.stdout).into_owned();
        if !text.trim().is_empty() {
            return Ok(text);
        }
        let renderer = resolve_executable(&["pdftoppm", "mutool"])?;
        let dir = std::env::temp_dir().join(format!("miad-pdf-{}", std::process::id()));
        let _ = fs::create_dir_all(&dir);
        let render = if renderer.file_name().and_then(|n| n.to_str()) == Some("pdftoppm") {
            std::process::Command::new(&renderer)
                .args(["-png", "-r", "200"])
                .arg(path)
                .arg(dir.join("page"))
                .output()
        } else {
            std::process::Command::new(&renderer)
                .args(["draw", "-r", "200", "-o"])
                .arg(dir.join("page-%d.png"))
                .arg(path)
                .output()
        };
        if let Err(e) = render {
            let _ = fs::remove_dir_all(&dir);
            return Err(format!("PDF page rendering failed: {e}"));
        }
        let mut pages: Vec<_> = fs::read_dir(&dir)
            .map_err(|e| e.to_string())?
            .filter_map(|e| e.ok().map(|x| x.path()))
            .collect();
        pages.sort();
        let mut all = String::new();
        for (i, p) in pages.iter().enumerate() {
            if cancel.load(Ordering::Relaxed) {
                let _ = fs::remove_dir_all(&dir);
                return Err("cancelled".into());
            }
            let ocr = resolve_executable(&["tesseract"])?;
            let r = std::process::Command::new(ocr)
                .arg(p)
                .arg("stdout")
                .output()
                .map_err(|e| format!("OCR unavailable: {e}"))?;
            if !r.status.success() {
                let _ = fs::remove_dir_all(&dir);
                return Err(format!("OCR failed on page {}", i + 1));
            }
            let s = String::from_utf8_lossy(&r.stdout);
            if !s.trim().is_empty() {
                all.push_str(&format!("\n\n--- Page {} ---\n\n{}", i + 1, s));
            }
        }
        let _ = fs::remove_dir_all(&dir);
        return if all.trim().is_empty() {
            Err("PDF OCR produced no text; review image or language configuration".into())
        } else {
            Ok(all)
        };
    }
    if matches!(
        ext.as_str(),
        "docx" | "docm" | "xlsx" | "xlsm" | "pptx" | "pptm" | "odt" | "ods" | "odp"
    ) {
        let file = fs::File::open(path).map_err(|e| format!("archive open failed: {e}"))?;
        let mut z = ZipArchive::new(file).map_err(|e| format!("archive parse failed: {e}"))?;
        let mut out = String::new();
        for i in 0..z.len() {
            let mut f = z.by_index(i).map_err(|e| e.to_string())?;
            let n = f.name().to_string();
            if n.ends_with(".xml")
                && (n.contains("word/")
                    || n.contains("sharedStrings")
                    || n.contains("sheet")
                    || n.contains("slides/")
                    || n == "content.xml")
            {
                let mut s = String::new();
                use std::io::Read;
                f.read_to_string(&mut s).ok();
                out.push_str(&xml_text(&s));
            }
        }
        if out.trim().is_empty() {
            return Err("archive contains no extractable text".into());
        }
        return Ok(out);
    }
    if matches!(ext.as_str(), "doc" | "dot" | "xls" | "xlt" | "ppt" | "pot") {
        let tool = resolve_executable(&["soffice", "libreoffice"])?; /* let tool = [
                                                                         "soffice",
                                                                         "libreoffice",
                                                                         "/Applications/LibreOffice.app/Contents/MacOS/soffice",
                                                                     ]
                                                                     .iter()
                                                                     .find(|candidate| {
                                                                         std::process::Command::new(**candidate)
                                                                             .arg("--version")
                                                                             .output()
                                                                             .is_ok()
                                                                     })
                                                                     .ok_or("Legacy Office requires LibreOffice/soffice; install it and retry")?; */
        let dir = std::env::temp_dir().join(format!("miad-office-{}", std::process::id()));
        let _ = fs::create_dir_all(&dir);
        let result = std::process::Command::new(tool)
            .args(["--headless", "--convert-to", "txt:Text", "--outdir"])
            .arg(&dir)
            .arg(path)
            .output()
            .map_err(|e| format!("LibreOffice launch failed: {e}"));
        let converted = dir
            .join(path.file_stem().unwrap_or_default())
            .with_extension("txt");
        let answer = match result {
            Ok(r) if r.status.success() => fs::read_to_string(&converted)
                .map_err(|e| format!("Legacy Office output missing: {e}")),
            Ok(r) => Err(format!(
                "Legacy Office conversion failed: {}",
                String::from_utf8_lossy(&r.stderr).trim()
            )),
            Err(e) => Err(e),
        };
        let _ = fs::remove_dir_all(&dir);
        return answer.and_then(|s| {
            if s.trim().is_empty() {
                Err("Legacy Office output was empty".into())
            } else {
                Ok(s)
            }
        });
    }
    if matches!(
        ext.as_str(),
        "png" | "jpg" | "jpeg" | "tif" | "tiff" | "bmp" | "webp"
    ) {
        let ocr = resolve_executable(&["tesseract"])?;
        let result = std::process::Command::new(ocr)
            .arg(path)
            .arg("stdout")
            .arg("--psm")
            .arg("3")
            .output()
            .map_err(|e| format!("OCR unavailable: install tesseract locally ({e})"))?;
        if !result.status.success() {
            return Err(format!(
                "OCR failed: {}",
                String::from_utf8_lossy(&result.stderr).trim()
            ));
        }
        let text = String::from_utf8_lossy(&result.stdout).into_owned();
        if text.trim().is_empty() {
            return Err("OCR produced no text; review image or language configuration".into());
        }
        return Ok(text);
    }
    if matches!(
        ext.as_str(),
        "txt" | "md" | "rst" | "csv" | "tsv" | "html" | "xhtml"
    ) {
        let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
        if text.trim().is_empty() {
            return Err("empty input; no output written".into());
        }
        return Ok(text);
    }
    Ok(format!("<!-- Phase 1 scaffold: adapter stub; not a real conversion -->\n\n# Conversion pending\n\nSource: `{}`\n\nNo engine is configured for this format.\n", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        sync::atomic::{AtomicU64, Ordering},
        time::{SystemTime, UNIX_EPOCH},
    };
    static NEXT: AtomicU64 = AtomicU64::new(0);
    fn td() -> PathBuf {
        let base = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        loop {
            let d = std::env::temp_dir().join(format!(
                "miad-{base}-{}",
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            if fs::create_dir(&d).is_ok() {
                return d;
            }
        }
    }
    #[test]
    fn scan_dedup_and_sort() {
        let d = td();
        fs::write(d.join("b.txt"), "x").unwrap();
        fs::write(d.join("a.bin"), "x").unwrap();
        let v = expand_inputs(&[d.join("b.txt")], &[d.clone()]).unwrap();
        assert_eq!(v.len(), 2);
        assert!(v[0] < v[1]);
        let _ = fs::remove_dir_all(d);
    }
    #[test]
    fn atomic_and_collision() {
        let d = td();
        let s = d.join("a.txt");
        fs::write(&s, "x").unwrap();
        let mut u = vec![];
        let p = output_path(&s, &[d.clone()], None, &mut u);
        atomic_write(&p, "ok").unwrap();
        let p2 = output_path(&s, &[d.clone()], None, &mut u);
        assert_ne!(p, p2);
        assert_eq!(fs::read_to_string(p).unwrap(), "ok");
        let _ = fs::remove_dir_all(d);
    }
    #[test]
    fn unsupported_is_honest() {
        let c = Arc::new(AtomicBool::new(false));
        assert!(stub(Path::new("x.bin"), &c).is_err());
    }
    #[test]
    fn direct_text_extraction_is_not_placeholder() {
        let d = td();
        let p = d.join("note.txt");
        fs::write(&p, "# Hello").unwrap();
        let c = Arc::new(AtomicBool::new(false));
        assert_eq!(stub(&p, &c).unwrap(), "# Hello");
        let _ = fs::remove_dir_all(d);
    }
}
