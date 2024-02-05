use crate::{events::file_read_event::FilesReadEvent, types::sftp_config::SftpConfig};
use std::net::TcpStream;
use std::io::{self, prelude::*};
use ssh2::Session;
use std::path::PathBuf;

#[derive(Debug)]
pub struct OnFileReadCommand {
    pub file_name: String

}

impl OnFileReadCommand {
    pub fn apply(&self, sftp_config: &SftpConfig) -> Result<FilesReadEvent, ssh2::Error> {
        let connection_string = format!("{}:{}", sftp_config.ip, sftp_config.port);
    
        let tcp = TcpStream::connect(connection_string).unwrap();
    
        let mut sess = Session::new()?;
        sess.set_tcp_stream(tcp);
        sess.handshake()?;
    
        sess.userauth_password(&sftp_config.username, &sftp_config.password)?;
    
        let mut path = PathBuf::new();
        path.push(&self.file_name);
    
        let mut sftp = sess.sftp()?;
        
        // Open the file
        let mut file = match sftp.open(&path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Error opening file: {}", err);
                return Err(err);
            }
        };
    
        // Read the content of the file into a Vec<u8>
        let mut buffer = Vec::new();
        if let Err(err) = file.read_to_end(&mut buffer) {
            eprintln!("Error reading file content: {}", err);
            return Err(ssh2::Error::eof());
        }
    
        // Create and return the FilesReadEvent
        let event = FilesReadEvent {
            file_name: self.file_name.clone(),
            file_content: String::from_utf8_lossy(&buffer).to_string().into(),
        };
    
        Ok(event)
    }
    
    }
