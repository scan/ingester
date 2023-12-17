// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;
mod event;
mod processing;

use futures::stream::StreamExt;
use std::fs;

use error::CopyError;

use crate::event::{CopyFinishedEvent, CopyStartEvent};

#[tauri::command]
fn directory_accessible(path: &str) -> bool {
    match fs::metadata(path) {
        Ok(meta) => meta.is_dir() && !meta.permissions().readonly(),
        Err(_) => false,
    }
}

#[tauri::command]
async fn copy_files(
    window: tauri::Window,
    source_path: &str,
    raw_path: &str,
    jpeg_path: &str,
    movie_path: &str,
    recursive: bool,
) -> Result<bool, CopyError> {
    log::info!("starting the copying");
    if let Err(err) = window.emit("copy_start", CopyStartEvent) {
        log::error!("failed to emit copy_start event: {}", err);
    }

    processing::copy_directory(
        window.clone(),
        source_path,
        raw_path,
        vec!["dng".to_owned(), "arw".to_owned(), "raf".to_owned()],
        recursive,
    )?
    .chain(processing::copy_directory(
        window.clone(),
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
    .chain(processing::copy_directory(
        window.clone(),
        source_path,
        movie_path,
        vec![
            "mp4".to_owned(),
            "mov".to_owned(),
            "avi".to_owned(),
        ],
        recursive,
    )?)
    .collect::<Vec<_>>()
    .await;

    log::info!("done with copying files");
    if let Err(err) = window.emit("copy_finished", CopyFinishedEvent) {
        log::error!("failed to emit copy_finished event: {}", err);
    }

    Ok(true)
}

fn main() {
    pretty_env_logger::init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![directory_accessible, copy_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
