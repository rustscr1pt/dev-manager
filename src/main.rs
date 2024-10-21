use crate::executor::execute::execute;

mod shell_executor;
mod connect_to_vps;
mod read_json_config;
mod executor;


fn main() -> () {
    match execute() {
        Ok(()) => {
            println!("Successfully executed.")
        }
        Err(err) => {
            println!("Couldn't find a configuration file\n{}", err)
        }
    }
}
