use crate::cli::{Cli, Commands};
use clap::Parser;
use log::{debug, info};

pub mod cli;
pub mod logger;

static TARGET: &str = "main";

fn main() {
    let cli = Cli::parse();

    // Initialize logger
    match logger::init(cli.get_log_level()) {
        Ok(()) => {
            debug!(target: TARGET, "Logger initialized successfully");
        }
        Err(err) => {
            panic!("Failed to initialize logger: {}", err);
        }
    };

    match &cli.command {
        Some(Commands::Library { sync, list }) => {
            if *list {
                info!(target: TARGET, "Listing games");
                return;
            }

            if *sync {
                info!(target: TARGET, "Syncing games");
                return;
            }
        }
        Some(Commands::Install { id }) => {
            info!(target: TARGET, "Installing {}", id);
        }
        None => {}
    }
}
