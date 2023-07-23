// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;
mod processing;

use futures::stream::StreamExt;
use std::fs;

use error::CopyError;

#[tauri::command]
fn directory_accessible(path: &str) -> bool {
    match fs::metadata(path) {
        Ok(meta) => meta.is_dir() && !meta.permissions().readonly(),
        Err(_) => false,
    }
}

#[tauri::command]
async fn copy_files(
    source_path: &str,
    raw_path: &str,
    jpeg_path: &str,
    recursive: bool,
) -> Result<bool, CopyError> {
    processing::copy_directory(
        source_path,
        raw_path,
        vec!["dng".to_owned(), "arw".to_owned(), "raf".to_owned()],
        recursive,
    )?
    .chain(processing::copy_directory(
        source_path,
        jpeg_path,
        vec![
            "avif".to_owned(),
            "hif".to_owned(),
            "jpeg".to_owned(),
            "jpg".to_owned(),
            "png".to_owned(),
            "tif".to_owned(),
            "tiff".to_owned(),
        ],
        recursive,
    )?)
    .collect::<Vec<_>>()
    .await;

    Ok(true)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![directory_accessible, copy_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}