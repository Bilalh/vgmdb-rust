use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub catalog: String,
    pub category: String,
    pub link: String,
    #[serde(rename = "media_format")]
    pub media_format: String,
    #[serde(rename = "release_date")]
    pub release_date: String,
    pub titles: Titles,
}

impl Album {
    /// Id of the album that can be given to `get_album`.
    pub fn album_id(&self) -> Option<u32> {
        match self.link.split('/').last().map(|x| x.parse::<u32>()) {
            Some(Ok(n)) => Some(n),
            _ => None,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Titles {
    #[serde(rename = "en")]
    pub english: String,
    #[serde(rename = "jp")]
    pub japenese: Option<String>,
    #[serde(rename = "ja-latn")]
    pub japenese_latin: String,
}
