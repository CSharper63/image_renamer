use chrono::{DateTime, Utc};
use cliclack::{input, intro, log, outro};
use std::{fs, path::Path, time::UNIX_EPOCH, usize};

// todo verify edge cases
fn rename_images(path: &str, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    let allowed_ext = vec!["jpg", "png", "jpeg", "gif", "tiff", "raw", "heic"];

    if path.is_empty() {
        return Err("Please provide a non empty path".into());
    }

    let Ok(dir) = Path::new(path).read_dir() else {
        return Err("Invalid path directory".into());
    };

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

        //let created_at = dt.format("%Y-%m-%d_%H-%M-%S").to_string();
        let created_at = dt.format("%Y-%m-%d").to_string();

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
        let mut duplicate_counter: usize = 0;
        while new_path.exists() {
            duplicate_counter += 1;
            new_name = format!("{}_{}.{}", created_at, duplicate_counter, ext);
            new_path = entry.path().with_file_name(new_name.clone());
        }

        // rename the file with its creation date
        fs::rename(entry.path(), new_path)?;
        element_processed_counter += 1;

        if !verbose {
            continue;
        }

        log::success(format!("{} renamed by {}", old_name, new_name.clone()))?;
    }

    log::success(format!(
        "Total renamed images: {}",
        element_processed_counter
    ))?;

    Ok(())
}

fn verify_path(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if path.is_empty() {
        return Err("Please provide a non empty path".into());
    }

    let Ok(_) = Path::new(path).read_dir() else {
        return Err("Invalid path directory".into());
    };

    Ok(())
}

fn req_user_for_path() -> Result<String, Box<dyn std::error::Error>> {
    let str = input("Where are your pictures located on your computper")
        .placeholder("path/to/my/pictures")
        .validate(|path: &String| verify_path(&path))
        .interact::<String>()?
        .to_string();
    Ok(str)
}

struct Params {
    verbose: bool,
    path: String,
}

impl Params {
    pub fn new(verbose: bool, path: String) -> Self {
        Params { verbose, path }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args();

    let mut params = Params::new(false, "".to_string());

    // handle params
    if args.len() <= 3 {
        let mut args = args.into_iter();
        args.next(); // start after prog name

        // image dir param
        let img_dir = args.next().unwrap_or("".to_string());
        params.path = img_dir.clone();

        // verbose param
        let verbose = args.next().unwrap_or("".to_string());
        params.verbose = verbose == "-v".to_string();
    }

    intro("Image renamer")?;

    if verify_path(&params.path).is_err() {
        let Ok(str) = req_user_for_path() else {
            return Err("Error occured".into());
        };
        params.path = str;
    };

    rename_images(&params.path, params.verbose)?;

    outro("Your pictures have been renamed successfully!")?;

    Ok(())
}
