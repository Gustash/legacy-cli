use once_cell::sync::Lazy;
use std::path::PathBuf;

pub static DEFAULT_CONFIG_DIR: Lazy<PathBuf> = Lazy::new(|| dirs::config_dir().unwrap().join("legacy-cli"));

pub static LIBRARY_JSON_FILENAME: &str = "library.json";
