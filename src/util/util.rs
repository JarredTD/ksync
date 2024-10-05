use std::fs::read_dir;
use std::io;
use std::path::{Path, PathBuf};

use crate::constants::INIT_DIR_NAME;

pub fn find_root(dir: &Path) -> Result<PathBuf, std::io::Error> {
    let entries = read_dir(dir)?;

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        if entry.file_type()?.is_dir() {
            if entry.file_name() == INIT_DIR_NAME {
                return Ok(entry.path());
            }
        }
    }
    match dir.parent() {
        Some(parent) => find_root(parent),
        None => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "no parent directory",
        )),
    }
}
