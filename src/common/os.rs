use std::path::Path;
use std::fs::File;

pub fn touch_file (path: &Path) -> Result<(), String> {
    match File::create(path) {
        Ok(_) => Ok(()),
        Err(io_error) => Err(format!("Failed to touch file {} (reason: {})", path.display(), io_error.to_string()))
    }
}