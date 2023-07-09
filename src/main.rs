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

pub enum LegacyError {
    IO(&'static str, std::io::Error),
    Reqwest(&'static str, reqwest::Error),
    StatusCode(&'static str, reqwest::StatusCode),
}

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
    let config = Config::new(&cli.config_path);

    if let Err(err) = handle_cli(&cli, &api, &config).await {
        handle_error(err);
    }
}

fn handle_error(error: LegacyError) {
    match error {
        LegacyError::IO(target, err) => error!(target: target, "IO Error: {}", err),
        LegacyError::Reqwest(target, err) => error!(target: target, "Reqwest error: {}", err),
        LegacyError::StatusCode(target, code) => {
            error!(target: target, "Request failed with code: {}", code)
        }
    }
}

async fn handle_cli(cli: &Cli, api: &API, config: &Config) -> Result<(), LegacyError> {
    match &cli.command {
        Some(Commands::Library { sync, list, email }) => {
            if *list {
                info!(target: TARGET, "Listing games");
            }

            if *sync {
                library::sync(&api, &config, email).await?;
            }
        }
        Some(Commands::Install { id }) => {
            info!(target: TARGET, "Installing {}", id);
            ()
        }
        None => {}
    };

    Ok(())
}
