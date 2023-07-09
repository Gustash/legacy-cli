use super::API;
use crate::{config::Config, models, LegacyError};
use log::info;

static TARGET: &str = "library";

pub async fn sync(api: &API, config: &Config, email: &String) -> Result<(), LegacyError> {
    info!(target: TARGET, "Syncing library");

    match api
        .get("/users/getgiveawaycatalogbyemail")
        .query(&[("email", email)])
        .send()
        .await
    {
        Ok(res) => handle_sync_reponse(res, config).await,
        Err(err) => Err(LegacyError::Reqwest(TARGET, err)),
    }
}

async fn handle_sync_reponse(res: reqwest::Response, config: &Config) -> Result<(), LegacyError> {
    if res.status() != reqwest::StatusCode::OK {
        return Err(LegacyError::StatusCode(TARGET, res.status()));
    }

    match res.json::<models::catalog::Response>().await {
        Ok(response) => {
            if response.status != "ok" {
                // TODO: Handle error
                return Ok(());
            }
            config.save_library(
                response
                    .data
                    .iter()
                    .flat_map(|x| &x.games)
                    .collect::<Vec<_>>(),
            )
        }
        Err(err) => Err(LegacyError::Reqwest(TARGET, err)),
    }
}
