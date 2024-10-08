use crate::config::config_object::Config;
use crate::constants::{CONF_FILE_NAME, INIT_DIR_NAME};
use crate::Args;
use clap::CommandFactory;
use std::fs;
use std::path::Path;

pub fn none() {
    Args::command().print_help().unwrap();
    println!();
}

pub fn init(current_dir: &Path) {
    fn write_config(current_dir: &Path, init_dir: &Path) {
        let config_file_path = init_dir.join(CONF_FILE_NAME);
        let config_file = Config::builder().build();
        match config_file.write(&config_file_path) {
            Ok(()) => {
                if let Some(parent) = current_dir.parent() {
                    if let Ok(relative_path) = config_file_path.strip_prefix(parent) {
                        log::debug!("Config file written to: {:?}", relative_path);
                    } else {
                        log::debug!("Config file written to: {:?}", config_file_path);
                    }
                } else {
                    log::debug!("Config file written to: {:?}", config_file_path);
                }
            }
            Err(e) => {
                log::error!("error writing config file: {}", e);
            }
        }
    }

    let init_dir = current_dir.join(INIT_DIR_NAME);
    match fs::create_dir(&init_dir) {
        Ok(()) => {
            write_config(current_dir, &init_dir);
            println!("ksync directory initiated");
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            println!("ksync already initialized");
        }
        Err(e) => {
            log::error!("error creating init directory: {}", e);
        }
    }
}

pub fn new() {
    unimplemented!()
}
