use chrono::{DateTime, Datelike, Month, Utc};
use futures::{stream, FutureExt, StreamExt};
use num_traits::cast::FromPrimitive;
use std::path::{Path, PathBuf};
use tokio::fs;

use crate::{error, event::CopyProgressEvent};

async fn process_file(
    window: tauri::Window,
    destination_path: impl AsRef<Path>,
    original_path: PathBuf,
) -> Result<(), error::CopyError> {
    let file_attributes = fs::metadata(&original_path).await?;
    let creation_time: DateTime<Utc> = file_attributes.created()?.into();

    log::debug!("start processing file {}", original_path.display());

    let new_file_name = original_path.clone();

    let year = creation_time.year();
    let month = creation_time.month();
    let month_name = Month::from_u32(month);
    let day = creation_time.day();

    let final_dir = destination_path
        .as_ref()
        .join(year.to_string())
        .join(format!(
            "{:0>2} - {}",
            month,
            month_name.map_or("Unknown", |m| m.name())
        ))
        .join(format!("{:0>4}-{:0>2}-{:0>2}", year, month, day));
    let final_path = final_dir.join(new_file_name.file_name().unwrap_or_default());

    log::debug!("determined target path: {}", final_path.display());

    if !final_dir.is_dir() {
        fs::create_dir_all(&final_dir).await?;
    }

    log::info!(
        "copying file {} to {}",
        original_path.display(),
        final_path.display()
    );
    fs::copy(&original_path, &final_path).await?;

    for ext in vec!["dop", "xmp"] {
        let fpath = PathBuf::from(format!("{}.{}", original_path.display(), ext));
        log::info!("Testing metadata name: {}", fpath.display());

        if fs::try_exists(&fpath).await? {
            let npath = final_dir.join(fpath.file_name().unwrap_or_default());

            log::info!(
                "also copying metadata file {} to {}",
                fpath.display(),
                npath.display()
            );

            fs::copy(&fpath, &npath).await?;
        }
    }

    window.emit(
        "copy_progress_done",
        CopyProgressEvent {
            file_name: final_path.file_name().unwrap().to_str().unwrap().to_owned(),
        },
    );

    Ok(())
}

pub fn copy_directory(
    window: tauri::Window,
    source_dir: impl AsRef<Path>,
    dest_dir: impl AsRef<Path>,
    file_extensions: Vec<String>,
    recursive: bool,
) -> Result<impl stream::Stream<Item = ()>, error::CopyError> {
    let file_pattern = format!(
        "*.{{{}}}",
        file_extensions
            .iter()
            .map(|s| s.to_lowercase())
            .collect::<Vec<String>>()
            .join(",")
    );

    let file_paths: Vec<PathBuf> =
        globwalk::GlobWalkerBuilder::from_patterns(source_dir, &[file_pattern])
            .follow_links(false)
            .max_depth(if recursive { 5 } else { 1 })
            .case_insensitive(true)
            .build()?
            .filter_map(Result::ok)
            .map(|img| img.into_path())
            .collect();

    let dest_dir = PathBuf::from(dest_dir.as_ref());
    Ok(stream::iter(file_paths)
        .map(move |path| process_file(window.clone(), dest_dir.clone(), path))
        .buffer_unordered(6)
        .for_each(|res| async {
            if let Err(e) = res {
                log::error!("failed to move file: {}", e);
            }
        })
        .into_stream())
}
