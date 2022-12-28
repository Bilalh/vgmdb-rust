use anyhow::Result;
use vgmdb::types::albums::Album;
use vgmdb::VgmdbClient;

#[tokio::main]
async fn main() -> Result<()> {
    let client = VgmdbClient::new();
    let data: Album = client.get_album(1547).await?;
    dbg!(data);

    Ok(())
}
