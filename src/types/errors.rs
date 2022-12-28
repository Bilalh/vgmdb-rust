use thiserror::Error;
#[derive(Error, Debug)]
pub enum VgmdbError {
    #[error("Network Error {0} ")]
    NetworkError(#[from] reqwest::Error),
    #[error("Invaild status code {0} ")]
    InvaildStatusCode(reqwest::StatusCode),
    #[error("Invaild data returned")]
    InvaildData,
}
