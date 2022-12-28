pub mod albums;
// We generated most of the structs using the follow based on the json files in data
// https://transform.tools/json-to-rust-serde
// Needed to edit it a bit because of overfitting.

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    // Test data from
    // parallel -j4 --tag "curl 'https://vgmdb.info/album/{}?format=json' > album_{}.json" ::: ids
    // parallel -j4 "echo '#[test_case(\"data/album_{}.json\")]' " ::: ids

    #[test_case("data/album_18.json")]
    #[test_case("data/album_885.json")]
    #[test_case("data/album_1547.json")]
    #[test_case("data/album_6445.json")]
    #[test_case("data/album_6988.json")]
    #[test_case("data/album_24725.json")]
    #[test_case("data/album_40854.json")]
    #[test_case("data/album_50946.json")]
    #[test_case("data/album_81114.json")]
    fn test_decoding_albums_from_json(filename: &str) -> () {
        let encoded = std::fs::read_to_string(filename).unwrap();
        serde_json::from_str::<albums::Album>(&encoded).unwrap();
    }

    // Bugs found by running it overs all my albums
    #[test_case("data/album_11999.json")]
    #[test_case("data/album_14966.json")]
    #[test_case("data/album_25985.json")]
    fn test_vaildation_set_decoding_albums_from_json(filename: &str) -> () {
        let encoded = std::fs::read_to_string(filename).unwrap();
        serde_json::from_str::<albums::Album>(&encoded).unwrap();
    }
}
