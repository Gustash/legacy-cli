use log::debug;

use crate::constants::{DEFAULT_CONFIG_DIR, LIBRARY_JSON_FILENAME};
use crate::models::catalog::Game;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

static TARGET: &str = "config";

pub struct Config {
    /// Directory where config files are stored
    config_path: PathBuf,
}

impl Config {
    pub fn new(path: Option<PathBuf>) -> Self {
        Self {
            config_path: match path {
                Some(path) => path,
                None => DEFAULT_CONFIG_DIR.to_owned(),
            },
        }
    }

    pub fn save_library(&self, games: Vec<&Game>) -> Result<(), std::io::Error> {
        let filepath = self.config_path.join(LIBRARY_JSON_FILENAME);
        debug!(target: TARGET, "File path: {:?}", filepath.as_os_str());
        let mut file = File::create(filepath)?;
        file.write_all(serde_json::to_string(&games).unwrap().as_bytes())
    }
}
