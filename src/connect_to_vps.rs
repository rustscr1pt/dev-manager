use std::io;
use std::net::TcpStream;
use ssh2::{Session};
use ssh2::Error as SshError;
use thiserror::Error;
use crate::command_executor::command_executor;

#[derive(Error, Debug)]
pub enum ConnectError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("SSH error: {0}")]
    Ssh(#[from] SshError),
    #[error("Custom error: {0}")]
    Custom(String)
}

pub fn connect_to_vps(user_name : String, password : String, host : String) -> Result<(), ConnectError> {
    match TcpStream::connect(format!("{}:22", host)) {
        Ok(tcp) => {
            match Session::new() {
                Ok(mut session) => {
                    session.set_tcp_stream(tcp);
                    match session.handshake() {
                        Ok(()) => {
                            match session.userauth_password(&user_name, &password) {
                                Ok(()) => {
                                    if session.authenticated() {
                                        println!("Authenticated");
                                        // command_executor("screen -X -S api kill", &session);
                                        // command_executor("./docker_clean.sh", &session);
                                        command_executor("screen -S api", &session);
                                        command_executor("cd api/main-api-docker ; docker-compose up --build", &session);
                                        command_executor("screen -d", &session);
                                        command_executor("screen -ls", &session);
                                        return Ok(())
                                    }
                                    else {
                                        return Err(ConnectError::Custom("Couldn't authenticate.".to_string()))
                                    }
                                }
                                Err(err) => {
                                    return Err(ConnectError::Ssh(err))
                                }
                            }
                        }
                        Err(err) => {
                            return Err(ConnectError::Ssh(err))
                        }
                    }
                }
                Err(err) => {
                    return Err(ConnectError::Ssh(err))
                }
            }
        }
        Err(err) => {
            return Err(ConnectError::Io(err))
        }
    }
}