use reqwest::{
    header::{ACCEPT, AUTHORIZATION, CACHE_CONTROL, CONTENT_TYPE},
    RequestBuilder,
};

pub mod library;

static BASE_URL: &str = "https://api.legacygames.com";

pub struct API {
    /// Reqwest client
    pub(super) client: reqwest::Client,
}

impl API {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    fn get(&self, url: &str) -> RequestBuilder {
        self.client
            .get(BASE_URL.to_owned() + &url)
            .header(AUTHORIZATION, "?token?")
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json")
            .header(CACHE_CONTROL, "no-cache")
    }
}
