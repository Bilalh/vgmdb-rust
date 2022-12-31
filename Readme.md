vgmdb
=====

Libary for https://vgmdb.net in rust.

Requires an async runtime.


## Example usage:

* Get album by its id.

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

* Search for albums by name

```rust
use vgmdb::{
    types::{search::Album, VgmdbError},
    VgmdbClient,
};

#[tokio::main]
async fn main() -> Result<(), VgmdbError>{
    let client = VgmdbClient::new();
    let data: Vec<Album> = client.search_albums("voice of a distant star").await?;
    dbg!(&data);
    dbg!(&data[0].album_id());

    Ok(())
}
```

