use anyhow::{bail, Result};
use reqwest;

use reqwest::header::{ACCEPT, CONTENT_TYPE};
use vgmdb::types::albums::Album;

#[tokio::main]
async fn main() -> Result<()> {
    let album_id = "81114";
    let url = format!("https://vgmdb.info/album/{album_id}?format=json");

    let client = reqwest::Client::new();
    let response = client
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
    let data = response.json::<Album>().await?;
    dbg!(data);

    Ok(())
}
