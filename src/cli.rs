use std::path::PathBuf;
use clap::{Parser, Subcommand};
use log::LevelFilter;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Enable debug logging
    #[arg(short, long)]
    pub debug: bool,

    /// Custom config directory
    #[arg(short, long)]
    pub config_path: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

impl Cli {
    pub fn get_log_level(&self) -> LevelFilter {
        if self.debug {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Library related commands
    Library {
        /// Sync library
        #[arg(short, long)]
        sync: bool,

        /// List games in library
        #[arg(short, long)]
        list: bool,

        /// Account email
        #[arg(short, long)]
        email: String,
    },

    /// Install a game in the library
    Install {
        #[arg()]
        id: String,
    },
}
