use actix_raft::{AppData, AppDataResponse, AppError};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Data {
    BlockHeightUpdate,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DataResponse {}

impl AppData for Data {}

impl AppDataResponse for DataResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(unspecified raft error)")
    }
}

impl std::error::Error for Error {}

impl AppError for Error {}
