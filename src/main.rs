use crate::api::API;
use crate::cli::{Cli, Commands};
use api::library;
use clap::Parser;
use config::Config;
use log::{debug, error, info};

mod api;
mod cli;
mod config;
mod constants;
mod logger;
mod models;

static TARGET: &str = "main";

#[tokio::main]
async fn main() {
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

    let api = API::new();
    let config = Config::new(cli.config_path);

    match &cli.command {
        Some(Commands::Library { sync, list, email }) => {
            if *list {
                info!(target: TARGET, "Listing games");
                return;
            }

            if *sync {
                match library::sync(&api, &config, email).await {
                    Ok(_) => (),
                    Err(err) => error!(target: TARGET, "Failed to sync library: {:?}", err),
                }
            }
        }
        Some(Commands::Install { id }) => {
            info!(target: TARGET, "Installing {}", id);
        }
        None => {}
    }
}
