mod connect_to_vps;
mod command_executor;
mod read_json_config;

use crate::connect_to_vps::{connect_to_vps};

fn main() -> () {

    match connect_to_vps(user, password, host) {
        Ok(()) => {
            return;
        }
        Err(err) => {
            println!("{}", err);
            return;
        }
    }
}
