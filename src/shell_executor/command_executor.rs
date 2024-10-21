use std::io;
use std::io::Read;
use ssh2::{Session};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandExecutorError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("SSH2 error: {0}")]
    Ssh(#[from] ssh2::Error),
}

pub fn command_executor(command : &str, session : &Session) -> Result<(), CommandExecutorError> {
    match session.channel_session() {
        Ok(mut channel) => {
            match channel.exec(command) {
                Ok(()) => {
                    let mut output : String = String::new();
                    match channel.read_to_string(&mut output) {
                        Ok(_) => {
                            println!("{}", output);
                            match channel.wait_close() {
                                Ok(()) => {
                                    return Ok(())
                                }
                                Err(err) => {
                                    return Err(CommandExecutorError::Ssh(err))
                                }
                            }
                        }
                        Err(err) => {
                            return Err(CommandExecutorError::Io(err))
                        }
                    }
                }
                Err(err) => {
                    return Err(CommandExecutorError::Ssh(err))
                }
            }
        }
        Err(err) => {
            return Err(CommandExecutorError::Ssh(err))
        }
    }
}