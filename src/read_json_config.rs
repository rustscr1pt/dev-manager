use std::fs::File;
use std::{env, io};
use std::io::{BufReader, Error};
use serde::Deserialize;
use thiserror::Error;
use crate::read_json_config::ReadJsonError::Io;

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
    let exe_path = env::current_exe()
        .map_err(|err| Io(err))?;
    let exe_dir = exe_path
        .parent()
        .ok_or(Io(Error::new(io::ErrorKind::Other, "Failed to get executable binary")))?;
    let path = exe_dir.join("vps_data.json");
    println!("{:?}", path);
    match File::open(path) {
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