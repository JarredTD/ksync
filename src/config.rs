use std::path::{Path, PathBuf};

const INIT_DIR_NAME: &str = ".ksync";

#[derive(Debug)]
pub struct Configurations {
    _root: PathBuf,
}

impl Configurations {
    pub fn _new(root_path: &Path) -> Self {
        Configurations {
            _root: PathBuf::from(root_path),
        }
    }

    pub fn default() -> Self {
        Configurations {
            _root: PathBuf::from(INIT_DIR_NAME),
        }
    }

    pub fn _get_root(&self) -> &Path {
        self._root.as_path()
    }
}
