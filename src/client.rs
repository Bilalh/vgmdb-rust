use crate::types::{albums::Album, search, VgmdbError};

use reqwest::header::{ACCEPT, CONTENT_TYPE};

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

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
        self.send_request(&url).await
    }

    /// Search vgmdb for albums matching the query
    pub async fn search_albums(&self, query: &str) -> Result<Vec<search::Album>, VgmdbError> {
        let url = format!("https://vgmdb.info/search/albums/{query}");
        let results: SearchData = self.send_request(&url).await?;

        Ok(results.results.albums)
    }

    async fn send_request<T>(&self, url: &str) -> Result<T, VgmdbError>
    where
        for<'a> T: Deserialize<'a>,
    {
        log::info!("Sending request to {url}");

        let response = self
            .client
            .get(url)
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .send()
            .await?;

        if reqwest::StatusCode::OK != response.status() {
            return Err(VgmdbError::InvaildStatusCode(response.status()));
        }

        let data = response.json::<T>().await.map_err(|e| {
            log::info!("Error returned for {url} {e}");

            VgmdbError::InvaildData
        })?;

        Ok(data)
    }
}

impl Default for VgmdbClient {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SearchData {
    pub link: String,
    pub meta: Value,
    pub query: String,
    pub results: SearchResults,
    pub vgmdb_link: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SearchResults {
    pub albums: Vec<search::Album>,
}
