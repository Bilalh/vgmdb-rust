use reqwest;

use reqwest::header::{ACCEPT, CONTENT_TYPE};

use crate::types::{albums::Album, VgmdbError};

/// Rust Client for vgmdb
pub struct VgmdbClient {
    client: reqwest::Client,
}

impl VgmdbClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_album(&self, album_id: u32) -> Result<Album, VgmdbError> {
        let url = format!("https://vgmdb.info/album/{album_id}?format=json");
        log::info!("Sending request to {url}");

        let response = self
            .client
            .get(&url)
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .send()
            .await?;

        if reqwest::StatusCode::OK != response.status() {
            return Err(VgmdbError::InvaildStatusCode(response.status()));
        }

        let data = response
            .json::<Album>()
            .await
            .map_err(|_| VgmdbError::InvaildData)?;

        Ok(data)
    }
}
