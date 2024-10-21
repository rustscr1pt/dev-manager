use thiserror::Error;
use crate::connect_to_vps::connect_to_vps::{connect_to_vps, ConnectError};
use crate::read_json_config::read_json_config::{read_json_config, ReadJsonError};

#[derive(Error, Debug)]
pub enum ExecuteError {
    #[error("IO error: {0}")]
    ConnectError(#[from] ConnectError),
    #[error("SerdeJson error: {0}")]
    SerdeJson(#[from] ReadJsonError)
}

pub fn execute() -> Result<(), ExecuteError> {
    match read_json_config() {
        Ok(data) => {
            match connect_to_vps(data.user, data.password, data.host) {
                Ok(()) => {
                    return Ok(());
                }
                Err(err) => {
                    return Err(ExecuteError::ConnectError(err));
                }
            }
        }
        Err(err) => {
            return Err(ExecuteError::SerdeJson(err))
        }
    }
}