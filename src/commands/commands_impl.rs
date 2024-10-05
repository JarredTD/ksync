use crate::config::config::Config;
use crate::constants::{CONF_FILE_NAME, INIT_DIR_NAME};
use crate::Args;
use clap::CommandFactory;
use std::fs;

pub fn none() {
    Args::command().print_help().unwrap();
    println!();
}

pub fn init() {
    if let Ok(current_dir) = std::env::current_dir() {
        let init_dir = current_dir.join(INIT_DIR_NAME);
        match fs::create_dir(&init_dir) {
            Ok(()) => {
                let config_file_path = init_dir.join(CONF_FILE_NAME);

                let config_file = Config::builder().build();
                match config_file.write(&config_file_path) {
                    Ok(()) => {
                        if let Some(parent) = current_dir.parent() {
                            if let Ok(relative_path) = config_file_path.strip_prefix(parent) {
                                println!("Config file written to: {:?}", relative_path);
                            } else {
                                println!("Config file written to: {:?}", config_file_path);
                            }
                        } else {
                            println!("Config file written to: {:?}", config_file_path);
                        }
                    }
                    Err(e) => {
                        eprintln!("error writing config file: {}", e);
                    }
                }
                println!("ksync directory initiated");
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                println!("ksync already initialized");
            }
            Err(e) => {
                eprintln!("error creating init directory: {}", e);
            }
        }
    }
}

pub fn new() {
    unimplemented!()
}
