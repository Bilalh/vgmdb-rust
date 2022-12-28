use anyhow::{bail, Result};
use reqwest;

use reqwest::header::{ACCEPT, CONTENT_TYPE};

use crate::types::albums::Album;

pub struct VgmdbClient {
    client: reqwest::Client,
}

impl VgmdbClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_album(&self, album_id: u32) -> Result<Album> {
        let url = format!("https://vgmdb.info/album/{album_id}?format=json");

        let response = self
            .client
            .get(&url)
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .send()
            .await?;

        if reqwest::StatusCode::OK != response.status() {
            bail!(format!(
                "Album request to {url} failed with code {}",
                response.status()
            ));
        }

        dbg!(url);
        let data = response.json::<Album>().await?;

        Ok(data)
    }
}
