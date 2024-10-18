use std::fs::File;
use std::io;
use std::io::BufReader;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct ParseJsonConfiguration {
    pub user : String,
    pub password : String,
    pub host : String
}

#[derive(Error, Debug)]
pub enum ReadJsonError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("SerdeJson error: {0}")]
    SerdeJson(#[from] serde_json::error::Error),
    #[error("Custom error: {0}")]
    Custom(String)
}

pub fn read_json_config() -> Result<ParseJsonConfiguration, ReadJsonError> {
    match File::open("vps_data.json") {
        Ok(file) => {
            let reader = BufReader::new(file);
            match serde_json::from_reader(reader) {
                Ok(data) => {
                    return Ok(data)
                }
                Err(err) => {
                    return Err(ReadJsonError::SerdeJson(err))
                }
            }
        }
        Err(err) => {
            return Err(ReadJsonError::Io(err))
        }
    }
}