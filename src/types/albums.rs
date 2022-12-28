use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub arrangers: Vec<Arranger>,
    pub barcode: Option<String>,
    pub catalog: String,
    pub categories: Vec<String>,
    pub category: String,
    pub classification: String,
    pub composers: Vec<Composer>,
    pub covers: Vec<Cover>,
    pub discs: Vec<Disc>,
    pub distributor: Option<Distributor>,
    pub link: Option<String>,
    pub lyricists: Vec<Value>,
    #[serde(rename = "media_format")]
    pub media_format: String,
    pub meta: Meta,
    pub name: String,
    pub names: Names,
    pub notes: String,
    pub organizations: Vec<Organization>,
    pub performers: Vec<Value>,
    #[serde(rename = "picture_full")]
    pub picture_full: String,
    #[serde(rename = "picture_small")]
    pub picture_small: String,
    #[serde(rename = "picture_thumb")]
    pub picture_thumb: String,
    pub platforms: Option<Vec<String>>,
    pub products: Vec<Product>,
    #[serde(rename = "publish_format")]
    pub publish_format: String,
    pub publisher: Publisher,
    pub related: Vec<Related>,
    #[serde(rename = "release_date")]
    pub release_date: String,
    #[serde(rename = "release_events")]
    pub release_events: Vec<Value>,
    #[serde(rename = "release_price")]
    pub release_price: ReleasePrice,
    pub reprints: Vec<Value>,
    #[serde(rename = "vgmdb_link")]
    pub vgmdb_link: String,
    pub votes: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Arranger {
    pub link: Option<String>,
    pub names: Names,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Composer {
    pub link: Option<String>,
    pub names: Names,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cover {
    pub full: String,
    pub medium: String,
    pub name: String,
    pub thumb: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Disc {
    #[serde(rename = "disc_length")]
    pub disc_length: String,
    pub name: String,
    pub tracks: Vec<Track>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub names: Names,
    #[serde(rename = "track_length")]
    pub track_length: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Distributor {
    pub link: Option<String>,
    pub names: Names,
    pub role: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    #[serde(rename = "added_date")]
    pub added_date: String,
    #[serde(rename = "edited_date")]
    pub edited_date: String,
    #[serde(rename = "fetched_date")]
    pub fetched_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    pub link: Option<String>,
    pub names: Names,
    pub role: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub link: Option<String>,
    pub names: Names,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Publisher {
    pub link: Option<String>,
    pub names: Names,
    pub role: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Related {
    pub catalog: String,
    pub date: Option<String>,
    pub link: Option<String>,
    pub names: Names,
    #[serde(rename = "type")]
    pub type_field: String,
}

// Manually fixed types

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum ReleasePrice {
    NormalPrice(NormalPrice),
    SpecialPrice(SpecialPrice),
    #[default]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NormalPrice {
    pub currency: String,
    pub price: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpecialPrice {
    pub price: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Names {
    #[serde(alias = "English", alias = "en")]
    pub english: Option<String>,
    #[serde(alias = "Japanese", alias = "jp")]
    pub japenese: Option<String>,
    #[serde(alias = "ja-latn")]
    pub japenese_latin: Option<String>,
}
