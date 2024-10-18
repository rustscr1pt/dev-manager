use std::io::Read;
use ssh2::Session;

pub fn command_executor(command : &str, session : &Session) -> () {
    let mut chanel = session.channel_session().unwrap();
    chanel.exec(command).unwrap();
    let mut output : String = String::new();
    chanel.read_to_string(&mut output).unwrap();
    println!("{}", output);
    chanel.wait_close().unwrap();
}