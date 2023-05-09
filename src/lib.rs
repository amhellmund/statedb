mod database;
mod common;

use std::path::PathBuf;

pub fn initialize_database (database_dir: &PathBuf, _config_file: &PathBuf) {
    println!("Initializing a new StateDB in directory: {}", database_dir.display());
    database::init::initialize(database_dir).expect("Database initialization");
}

pub fn launch_database (database_dir: &PathBuf, address: &str, port: i16) {
    if !database::init::is_initialized(database_dir) {
        panic!("The database is not yet initialized in directory: {}", database_dir.display());
    }
    println!("Launching StateDB on address {} and port {}", address, port);
}

pub fn open_console (_port: i16, _user: &str, _password: &str) {
    println!("Welcome to StateDB Console")
}