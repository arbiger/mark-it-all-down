#[cfg_attr(mobile, tauri::mobile_entry_point)]
use std::{path::PathBuf, sync::{Arc, atomic::{AtomicBool, Ordering}}};
use serde::Serialize;

#[tauri::command]
fn path_kind(path: String) -> Result<&'static str, String> { let p=PathBuf::from(path); if p.is_dir(){Ok("folder")}else if p.is_file(){Ok("file")}else{Err("Dropped path does not exist".into())} }

#[derive(Serialize)] struct QueueItem { path: String, extension: String, supported: bool, output: String }
#[derive(Serialize)] struct ResultItem { path: String, status: String, output: Option<String> }
#[tauri::command] fn phase_status() -> &'static str { "Local adapters: text, PDF, image OCR, OOXML/ODF, and optional LibreOffice legacy Office" }
#[tauri::command] fn plan_queue(files: Vec<String>, folders: Vec<String>, chosen_output: Option<String>) -> Result<Vec<QueueItem>, String> { let fs:Vec<_>=files.into_iter().map(PathBuf::from).collect(); let ds:Vec<_>=folders.into_iter().map(PathBuf::from).collect(); let all=mark_it_all_down::expand_inputs(&fs,&ds).map_err(|e|e.to_string())?; let roots=ds.iter().map(|p|std::fs::canonicalize(p).unwrap_or(p.clone())).collect::<Vec<_>>(); let mut used=vec![]; Ok(all.into_iter().map(|p|{let o=mark_it_all_down::output_path(&p,&roots,chosen_output.as_deref().map(std::path::Path::new),&mut used);QueueItem{path:p.display().to_string(),extension:p.extension().and_then(|x|x.to_str()).unwrap_or("").to_string(),supported:mark_it_all_down::supported(&p),output:o.display().to_string()}}).collect()) }
#[tauri::command] fn convert_queue(paths: Vec<String>, outputs: Vec<String>, cancel: tauri::State<'_, Arc<AtomicBool>>) -> Vec<ResultItem> { cancel.store(false, Ordering::Relaxed); paths.into_iter().zip(outputs).map(|(p,o)|{let path=PathBuf::from(p);match mark_it_all_down::stub(&path,&cancel){Ok(body)=>match mark_it_all_down::atomic_write(&PathBuf::from(&o),&body){Ok(_)=>{let ext=path.extension().and_then(|x|x.to_str()).unwrap_or("").to_ascii_lowercase();let status=if matches!(ext.as_str(),"png"|"jpg"|"jpeg"|"tif"|"tiff"|"bmp"|"webp"){ "OCR converted" }else if matches!(ext.as_str(),"txt"|"md"|"rst"|"csv"|"tsv"|"html"|"xhtml"|"pdf"){ "converted" }else{"unsupported adapter"};ResultItem{path:path.display().to_string(),status:status.into(),output:Some(o)}},Err(e)=>ResultItem{path:path.display().to_string(),status:e.to_string(),output:None}},Err(e)=>ResultItem{path:path.display().to_string(),status:e,output:None}}}).collect() }
#[tauri::command] fn cancel(cancel: tauri::State<'_, Arc<AtomicBool>>) -> bool { cancel.store(true, Ordering::Relaxed); true }

pub fn run() {
  let cancel_token = Arc::new(AtomicBool::new(false));
  tauri::Builder::default().manage(cancel_token)
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![phase_status, plan_queue, convert_queue, cancel, path_kind])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
