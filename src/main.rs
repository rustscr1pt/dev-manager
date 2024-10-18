mod connect_to_vps;
mod command_executor;
mod read_json_config;

use crate::connect_to_vps::{connect_to_vps};
use crate::read_json_config::{read_json_config};

fn main() -> () {
    match read_json_config() {
        Ok(data) => {
            match connect_to_vps(data.user, data.password, data.host) {
                Ok(()) => {
                    println!("Successfully executed");
                    return;
                }
                Err(err) => {
                    println!("{}", err);
                    return;
                }
            }
        }
        Err(err) => {
            println!("{}", err)
        }
    }
}
