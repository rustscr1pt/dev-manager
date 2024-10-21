use crate::connect_to_vps::connect_to_vps::connect_to_vps;
use crate::read_json_config::read_json_config::{read_json_config, ReadJsonError};

pub fn execute() -> Result<(), ReadJsonError> {
    match read_json_config() {
        Ok(data) => {
            match connect_to_vps(data.user, data.password, data.host) {
                Ok(()) => {
                    println!("Successfully executed");
                    return Ok(());
                }
                Err(err) => {
                    println!("{}", err);
                    return Ok(());
                }
            }
        }
        Err(err) => {
            return Err(err)
        }
    }
}