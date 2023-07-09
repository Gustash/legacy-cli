use super::API;
use crate::{config::Config, models};
use log::{error, info};

static TARGET: &str = "library";

// FIXME: Better error handling

#[derive(Debug)]
pub enum LibraryError {
    IO(std::io::Error),
    Request(reqwest::Error),
    StatusCode(reqwest::StatusCode),
}

pub async fn sync(api: &API, config: &Config, email: &String) -> Result<(), LibraryError> {
    info!(target: TARGET, "Syncing library");

    match api
        .get("/users/getgiveawaycatalogbyemail")
        .query(&[("email", email)])
        .send()
        .await
    {
        Ok(res) => handle_sync_reponse(res, config).await,
        Err(err) => {
            error!(target: TARGET, "Failed to fetch library: {}", err);
            Err(LibraryError::Request(err))
        }
    }
}

async fn handle_sync_reponse(res: reqwest::Response, config: &Config) -> Result<(), LibraryError> {
    match res.status() {
        reqwest::StatusCode::OK => match res.json::<models::catalog::Response>().await {
            Ok(response) => {
                if response.status != "ok" {
                    // TODO: Handle error
                    return Ok(());
                }
                match config.save_library(
                    response
                        .data
                        .iter()
                        .flat_map(|x| &x.games)
                        .collect::<Vec<_>>(),
                ) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(LibraryError::IO(err)),
                }
            }
            Err(err) => {
                error!(target: TARGET, "Failed to parse API response");
                Err(LibraryError::Request(err))
            }
        },
        code => {
            error!(target: TARGET, "Failed to fetch library: {:?}", code);
            Err(LibraryError::StatusCode(code))
        }
    }
}
