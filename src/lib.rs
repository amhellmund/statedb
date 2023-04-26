use std::path::PathBuf;

pub fn initialize_database (database_dir: &PathBuf, _config_file: &PathBuf) {
    println!("Initializing a new StateDB in directory: {}", database_dir.display());
}

pub fn launch_database (_database_dir: &PathBuf, address: &str, port: i16) {
    println!("Launching StateDB on address {} and port {}", address, port);
}

pub fn open_console (_port: i16, _user: &str, _password: &str) {
    println!("Welcome to StateDB Console")
}