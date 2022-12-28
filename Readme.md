vgmdb
=====

Libary for https://vgmdb.net in rust.

Requires an async runtime.


Example usage:


```rust
use vgmdb::{
    types::{albums::Album, VgmdbError},
    VgmdbClient,
};

#[tokio::main]
async fn main() -> Result<(), VgmdbError> {
    let client = VgmdbClient::new();
    let data: Album = client.get_album(1547).await?;
    dbg!(data);

    Ok(())
}
```