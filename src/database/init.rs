use std::path::{Path,PathBuf};


fn get_initialized_file (database_dir: &Path) -> PathBuf {
    database_dir.join(".init")
}

pub fn initialize(database_dir: &Path) -> Result<(), String> {
    let init_file = get_initialized_file(database_dir);
    crate::common::os::touch_file(&init_file)
}

pub fn is_initialized (database_dir: &Path) -> bool {
    let init_file = get_initialized_file(database_dir);
    init_file.exists() && init_file.is_file()
}

pub fn lock_database (database_dir: &Path) -> Result<(), String> {
    let lock_file = database_dir.join(".lock_service");
    crate::common::os::touch_file(&lock_file)?;
    Ok(())
}