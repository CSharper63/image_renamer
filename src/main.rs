use std::{fs, path::Path, time::UNIX_EPOCH};

use chrono::{DateTime, Utc};

// todo verify edge cases
fn rename_images(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let allowed_ext = vec!["jpg", "png", "jpeg", "gif", "tiff", "raw"];

    if path.is_empty() {
        return Err("Please provide a non empty path".into());
    }

    let Ok(dir) = Path::new(path).read_dir() else {
        return Err("Invalid path directory".into());
    };

    let mut duplicate_counter: u32 = 0;
    let mut element_processed_counter: usize = 0;

    for entry in dir {
        let Ok(entry) = entry else {
            return Err("Invalid file".into());
        };

        let metadata = entry.metadata()?;

        if !metadata.is_file() {
            continue;
        };

        // handle unix time to human readable
        let Ok(sec) = metadata.created()?.duration_since(UNIX_EPOCH) else {
            return Err("Cannot convert SystemTime to Duration".into());
        };

        let Some(dt) = DateTime::<Utc>::from_timestamp(sec.as_secs() as i64, 0) else {
            return Err("Cannot convert to DateTime".into());
        };

        let created_at = dt.format("%Y-%m-%d_%H-%M-%S").to_string();

        // extract current name with extension
        let path = entry.path();
        let (Some(old_name), Some(ext)) = (path.file_name(), Path::new(&path).extension()) else {
            return Err("Cannot extract filename and extension".into());
        };

        let old_name = old_name.to_string_lossy().to_string();
        let ext = ext.to_string_lossy().to_string().to_lowercase();

        // verify the file extension
        if !allowed_ext.contains(&ext.as_str()) {
            continue;
        }

        // build new name based on the creation date
        let mut new_name = format!("{}.{}", created_at, ext);
        let mut new_path = entry.path().with_file_name(new_name.clone());

        // verify duplicate
        // handle case if existing, avoid erasing file with same name
        while new_path.exists() {
            duplicate_counter += 1;
            new_name = format!("{}_{}.{}", created_at, duplicate_counter, ext);
            new_path = entry.path().with_file_name(new_name.clone());
        }

        // rename the file with its creation date
        fs::rename(entry.path(), new_path)?;
        element_processed_counter += 1;
        println!("{} renamed by {}", old_name, new_name.clone());
    }

    println!("Total renamed images: {}", element_processed_counter);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    while args.len() != 2 {
        println!("Please provide the dir path which contains your images: ");
        println!("Usage: image_renamer <img_dir_path>");
    }

    let dir_path = &args[1];

    rename_images(&dir_path)?;

    Ok(())
}
